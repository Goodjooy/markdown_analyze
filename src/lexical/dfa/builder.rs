use std::ops::Not;

use super::{
    super::tokens::{
        code_snippet::PartCodeSnippet,
        titles::{Title1, Title2, Title3, Title4, Title5, Title6},
        Trans,
    },
    core::DFA,
    utils::AdHocCanAny,
    wraps::{AnyType, InputChar, Status},
};

use self::trans_holder::TransHolder;

use crate::lexical::tokens::{
    BoxEnd, BoxMid, ChangeLine, Idented, ImgStart, LinkStart, NewParam, OrderList, Reference,
    SepChar, SepLine, Star, UnorderList,
};

mod trans_holder;

pub struct DFABuilder {
    inner: DFA,
    set_init: bool,
    set_start: bool,
    init_count: usize,
}

impl DFABuilder {
    pub fn new(init_status: usize, line_start: usize) -> Self {
        Self {
            inner: DFA::new(),
            set_init: false,
            set_start: false,
            init_count: init_status.max(line_start),
        }
        .add_init_status(init_status)
        .add_line_start_status(line_start)
    }

    fn add_init_status(mut self, status: usize) -> Self {
        if !self.set_init {
            self.inner.init_status = Status(status);
            self.set_init = true;
        }
        self
    }

    fn add_line_start_status(mut self, status: usize) -> Self {
        if !self.set_start && self.set_init {
            self.inner.line_start = Status(status);
            self.set_start = true;

            self.inner.table.insert(
                (InputChar::LineStart, self.inner.init_status.clone()),
                Status(status),
            );
        }
        self
    }

    pub fn add_trans<F>(mut self, handle: F) -> Self
    where
        F: FnOnce(&mut TransHolder),
    {
        let mut holder = TransHolder::new(
            self.init_count + 1,
        );

        handle(&mut holder);

        self.init_count = holder.get_counter();

        let TransHolder {
            trans,
            any_trans,
            ac_status,
            ..
        } = holder;

        self.inner.table.extend(
            trans
                .into_iter()
                .map(|((sk, ik), v)| ((ik, sk.into()), v.into())),
        );

        self.inner
            .any_trans
            .extend(any_trans.into_iter().map(|(k, v)| (k.into(), v)));

        self.inner
            .final_status
            .extend(ac_status.into_iter().map(|(k, v)| (k.into(), v)));
        self
    }

    pub fn build(self) -> DFA {
        self.inner
    }

    pub fn init() -> DFA {
        Self::new(0, 1)
            .add_trans(|h| {
                // title
                let c = '#';
                //#
                let mut next = h.add_tran_with_auto_next(1, c);
                h.set_accept_status(next, Title1);
                //##
                next = h.add_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title2);
                //###
                next = h.add_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title3);
                //####
                next = h.add_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title4);
                //#####
                next = h.add_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title5);
                //######
                next = h.add_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title6);
                //refer
                next = h.add_tran_with_auto_next(1, '>');
                next = h.add_tran(next, '>', next);
                h.set_accept_status(next, Reference);
                //link start
                next = h.add_tran_with_auto_next(0, '[');
                next = h.add_tran(1, '[', next);
                h.set_accept_status(next, LinkStart);
                //image start
                next = h.add_tran_with_auto_next(0, '!');
                next = h.add_tran(1, '!', next);
                next = h.add_tran_with_auto_next(next, '[');
                h.set_accept_status(next, ImgStart);
                //box mid
                next = h.add_tran_with_auto_next(0, ']');
                next = h.add_tran_with_auto_next(next, '(');
                h.set_accept_status(next, BoxMid);
                // box end
                next = h.add_tran_with_auto_next(0, ')');
                h.set_accept_status(next, BoxEnd);
                // unorder list
                next = h.add_tran_with_auto_next(1, '-');
                let m1 = next;
                next = h.add_tran_with_auto_next(1, '*');
                let m2 = next;
                h.set_accept_status(m1, UnorderList);
                h.set_accept_status(m2, UnorderList);
                // order list
                h.add_can_any(next, AdHocCanAny::new(|c| AnyType::Digit.type_match(c)));
                next = h.add_tran_with_auto_next(next, AnyType::Digit);
                next = h.add_tran_with_auto_next(next, '.');
                h.set_accept_status(next, OrderList);
                // sep line
                next = h.add_tran_with_auto_next(1, '-');
                next = h.add_tran_with_auto_next(next, '-');
                next = h.add_tran_with_auto_next(next, '-');
                h.set_accept_status(next, SepLine);
                next = h.add_tran_with_auto_next(1, '*');
                next = h.add_tran_with_auto_next(next, '*');
                next = h.add_tran_with_auto_next(next, '*');
                h.set_accept_status(next, SepLine);
                //star
                next = h.add_tran_with_auto_next(0, '*');
                h.set_accept_status(next, Star);
                next = h.add_tran(1, '*', next);
                next = h.add_tran_with_auto_next(next, '*');
                h.set_accept_status(next, Star);

                //specal type
                //sep
                next = h.add_tran_with_auto_next(0, ' ');
                h.set_accept_status(next, SepChar);
                // // line change
                next = h.add_tran_with_auto_next(0, '\n');
                next = h.add_tran(1, '\n', next);
                h.set_accept_status(next, ChangeLine);
                // // parghe change
                next = h.add_tran_with_auto_next(0, ' ');
                next = h.add_tran_with_auto_next(next, ' ');
                next = h.add_tran_with_auto_next(next, '\n');
                h.set_accept_status(next, NewParam);
                // //idented
                next = h.add_tran_with_auto_next(1, ' ');
                next = h.add_tran_with_auto_next(next, ' ');
                next = h.add_tran_with_auto_next(next, ' ');
                next = h.add_tran_with_auto_next(next, ' ');
                next = h.add_tran(1, '\t', next);
                h.set_accept_status(next, Idented);

                // 转义符
                next = h.add_tran_with_auto_next(0, '\\');
                next = h.add_tran(1, '\\', next);
                let ts = next;
                next = h.add_tran_with_auto_next(next, '\\');
                next = h.add_tran(ts, '#', next);
                next = h.add_tran(ts, '`', next);
                next = h.add_tran(ts, '-', next);
                next = h.add_tran(ts, '-', next);
                next = h.add_tran(ts, '{', next);
                next = h.add_tran(ts, '}', next);
                next = h.add_tran(ts, '[', next);
                next = h.add_tran(ts, ']', next);
                next = h.add_tran(ts, '(', next);
                next = h.add_tran(ts, ')', next);
                next = h.add_tran(ts, '+', next);
                next = h.add_tran(ts, '.', next);
                next = h.add_tran(ts, '!', next);
                next = h.add_tran(ts, '|', next);
                h.set_accept_status(next, Trans);

                // 简短代码块
                // 空白内容
                next = h.add_tran_with_auto_next(0, '`');
                next = h.add_tran(1, '`', next);
                let inner_start = next;
                next = h.add_tran_with_auto_next(next, '`');
                let inner_end = next;
                // 空白代码接受位置
                h.set_accept_status(next, PartCodeSnippet);
                // 非空白部分
                // 内部接受任何非'`'字符
                h.add_can_any(
                    inner_start,
                    AdHocCanAny::new(|buff| (!AnyType::Char('\n')).type_match(buff)),
                );
                next = h.add_tran_with_auto_next(inner_start, !AnyType::Char('\n'));
                h.add_can_any(
                    next,
                    AdHocCanAny::new(|input| AnyType::Any.type_match(input)),
                );
                next = h.add_tran(next, AnyType::Any, next);
                next = h.add_tran_with_auto_next(next, '`');
                // 简短代码块接受位置
                h.set_accept_status(next, PartCodeSnippet);
                // 后续复杂代码块
                next = h.add_tran_with_auto_next(inner_end, '`');
                h.add_can_any(
                    next,
                    AdHocCanAny::new(|input| AnyType::Any.type_match(input)),
                );
                next = h.add_tran(next, AnyType::Any, next);
                let inner = next;

                next = h.add_tran_with_auto_next(next, '`');
                h.add_can_any(
                    next,
                    AdHocCanAny::new(|input| AnyType::Any.type_match(input)),
                );
                h.add_tran(next, AnyType::Any, inner);

                next = h.add_tran_with_auto_next(next, '`');
                h.add_can_any(
                    next,
                    AdHocCanAny::new(|input| AnyType::Any.type_match(input)),
                );
                h.add_tran(next, AnyType::Any, inner);
                next = h.add_tran_with_auto_next(next, '`');
                h.set_accept_status(next, PartCodeSnippet);
            })
            .build()
    }
}

#[cfg(test)]
mod test {

    use full_token_derive_macro::FullToken;

    use crate::lexical::token_trait::{FullToken, TokenTrait};

    use super::*;

    #[derive(FullToken, Debug, Clone)]
    struct Indented;

    impl TokenTrait for Indented {
        fn name(&self) -> &'static str {
            "Idented"
        }

        fn to_full(&self, _: &[char]) -> Box<dyn FullToken> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn test_full_dfa() {
        let dfa = DFABuilder::init();

        print!("{}", dfa);
    }
}
