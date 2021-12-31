use std::str::Chars;

use serde::de::IntoDeserializer;

use self::{
    dfa::{
        builder::DFABuilder,
        core::DFA,
        wraps::{InputChar, Status},
    },
    token_trait::FullToken,
    tokens::{Eof, Plain},
};

mod dfa;
mod nfa;
pub mod token_trait;
pub mod tokens;

type RawData = Vec<char>;

pub struct Token(Box<dyn FullToken>, String);

pub struct LexicalLoader<'s> {
    // 输入的字符
    input: Chars<'s>,
    // 自动机
    dfa: DFA,
    // 状态区
    status: Status,
    // 临时状态机输入暂存
    to_dfa: Option<InputChar>,
}

impl<'s> LexicalLoader<'s> {
    pub fn new(input: Chars<'s>) -> Self {
        let dfa = DFABuilder::init();
        Self {
            input,
            status: dfa.init(),
            to_dfa: None,
            dfa,
        }
    }
}
impl<'s> LexicalLoader<'s> {
    pub fn next_token(&mut self) -> Token {
        // load next input char
        let mut status = self.status;
        let mut plain_buff = Vec::with_capacity(128);
        loop {
            let next_input = self.next_char();
            // println!("now_status: {} input {:?}",status,next_input);
            // 状态机启动
            match self.dfa.next_status(status, next_input.clone()) {
                // 中间状态，继续转换
                dfa::wraps::NextStatus::GoOn(s) => {
                    // 如果纯文本buff不为空，有前置纯文本
                    // 保持状态
                    if plain_buff.len() != 0 {
                        self.to_dfa = Some(next_input);
                        self.dfa.reset();
                        break Token(
                            Plain::new_box(&plain_buff),
                            plain_buff.into_iter().collect(),
                        );
                    } else {
                        status = s
                    }
                }
                // 终结状态,状态机停机
                dfa::wraps::NextStatus::Final(ft, pl, s, i) => {
                    let token = Token(ft, pl.into_iter().collect());
                    // 重置状态机状态
                    self.status = s;
                    self.to_dfa = Some(i);
                    break token;
                }
                dfa::wraps::NextStatus::Plain(pl, s, i) => {
                    // 普通文本，没进buff就停机
                    // println!("plain{:?} ,{},{}",&pl,&s,i);
                    if pl.len() == 0 {
                        if let InputChar::Char(c) = i {
                            plain_buff.push(c);
                        }
                        // 输入结束，如果buff 非空，就返回
                        else if let InputChar::Eof = i {
                            let token = Token(
                                if plain_buff.len() == 0 {
                                    Box::new(Eof)
                                } else {
                                    Plain::new_box(&plain_buff)
                                },
                                plain_buff.into_iter().collect(),
                            );
                            break token;
                        } else {
                            // 输入必须是普通文本
                            unreachable!()
                        }
                        self.status = s;
                        self.to_dfa = None;
                    } else {
                        let token = Token(
                            Plain::new_box(&plain_buff),
                            pl.into_iter().collect(),
                        );
                        self.status = s;
                        self.to_dfa = Some(i);
                        break token;
                    }
                }
            }
        }
    }

    fn next_char(&mut self) -> InputChar {
        if let Some(c) = self.to_dfa.clone() {
            self.to_dfa = None;
            c
        } else if let Some(c) = self.input.next() {
            c.into()
        } else {
            InputChar::Eof
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexical::token_trait::FromTokenMeta;

    use super::*;

    #[test]
    fn test_read_plain() {
        let input = String::from("这里是凊弦凝绝~ this is Frozen String");
        let mut lex = LexicalLoader::new(input.chars());

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "这里是凊弦凝绝~");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "this");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "is");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "Frozen");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "String");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "");
    }

    #[test]
    fn test_title_read() {
        let input="## Md解析器 ***全新版本*** 好耶 [abab](http://www.bilibili b站) emm\n\n好家伙，这么顶  \n\n";
        let mut lex = LexicalLoader::new(input.chars());

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "##");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "Md解析器");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "全新版本");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "好耶");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "[");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "abab");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "](");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "http://www.bilibili");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "b站");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, ")");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "emm");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "好家伙，这么顶");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "  \n");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        //eof
        assert_eq!(r, "");
    }

    #[test]
    fn test_code_snip() {
        let input = "`rust`and```rus`tc```for the ```mirai`rust````";
        let mut lex = LexicalLoader::new(input.chars());

        let Token(t, r) = lex.next_token();
        println!("{:?} - {:?}", t.get_data("inner"), r);
        assert_eq!(
            "rust",
            String::from_token_meta(&t.get_data("inner").unwrap()).unwrap()
        );

        let Token(t, r) = lex.next_token();
        println!("{} - {:?}", t.name(), r);
        assert_eq!("and", r);

        let Token(t, r) = lex.next_token();
        println!("{:?} - {:?}", t.get_data("inner"), r);
        assert_eq!(
            "rus`tc",
            String::from_token_meta(&t.get_data("inner").unwrap()).unwrap()
        );

        let Token(t, r) = lex.next_token();
        println!("{} - {:?}", t.name(), r);
        assert_eq!("for", r);

        let Token(t, r) = lex.next_token();
        println!("{} - {:?}", t.name(), r);
        assert_eq!(" ", r);

        let Token(t, r) = lex.next_token();
        println!("{} - {:?}", t.name(), r);
        assert_eq!("the", r);

        let Token(t, r) = lex.next_token();
        println!("{} - {:?}", t.name(), r);
        assert_eq!(" ", r);

        let Token(t, r) = lex.next_token();
        println!("{:?} - {:?}", t.get_data("inner"), r);
        assert_eq!(
            "mirai`rust",
            String::from_token_meta(&t.get_data("inner").unwrap()).unwrap()
        );

        let Token(t, r) = lex.next_token();
        println!("{} - {:?}", t.name(), r);
        assert_eq!("`", r);
    }

    #[test]
    fn test_list() {
        let input = "* abbabb\n* bbcbxx\n    12222332. abbaab\n    >> emmc?\n        codeblock";
        let mut lex = LexicalLoader::new(input.chars());

        // 行首无序段落
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "abbabb");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");

        // 第二行 无序段落
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "*");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "bbcbxx");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");

        // 缩进段落
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "    ");
        // 有序列表
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "12222332.");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "abbaab");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");

        // 缩进段落
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "    ");

        //引用 二级
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, ">>");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, " ");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "emmc?");

        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "\n");

        // 缩进段落
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "    ");

        // 缩进段落
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "    ");
        //二级缩进
        let Token(_t, r) = lex.next_token();
        println!("token raw: {:?}", r);
        assert_eq!(r, "codeblock");
    }
}
