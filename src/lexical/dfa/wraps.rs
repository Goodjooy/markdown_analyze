use std::ops::{BitAnd, BitOr, Not};

use super::super::token_trait::FullToken;

#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Status(pub(super) usize);

impl From<usize> for Status {
    fn from(i: usize) -> Self {
        Self(i)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AnyType {
    // any char
    Any,
    // digit char
    Digit,
    // 字母表
    Alphabet,
    // 小写
    LowerCase,
    // 大写
    UpperCase,
    // 空白
    WhiteSpace,
    // 是个数字
    Numer,
    // 是否为ASCII编码
    Ascii,
    // char
    Char(char),
    // 其他类型（要提供标记id）
    Orther(usize),
    // 混合
    Or(Box<AnyType>, Box<AnyType>),
    And(Box<AnyType>, Box<AnyType>),
    Not(Box<AnyType>),
}

impl AnyType {
    fn is_match(&self, input: char) -> bool {
        match self {
            AnyType::Any => true,
            AnyType::Digit => input.is_digit(10),
            AnyType::Alphabet => input.is_alphabetic(),
            AnyType::LowerCase => input.is_lowercase(),
            AnyType::UpperCase => input.is_uppercase(),
            AnyType::WhiteSpace => input.is_whitespace(),
            AnyType::Numer => input.is_numeric(),
            AnyType::Ascii => input.is_ascii(),
            AnyType::Char(c) => &input == c,
            // orhter 需要手动实现类型匹配判断
            AnyType::Orther(_) => false,
            AnyType::Or(l, r) => l.is_match(input) || r.is_match(input),
            AnyType::And(l, r) => l.is_match(input) && r.is_match(input),
            AnyType::Not(s) => !s.is_match(input),
        }
    }

    pub fn type_match(&self, input: char) -> Option<Self> {
        if self.is_match(input) {
            Some(self.clone())
        } else {
            None
        }
    }
}

impl From<char> for AnyType {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl Into<InputChar> for AnyType {
    fn into(self) -> InputChar {
        InputChar::Any(self)
    }
}

impl BitOr for AnyType {
    type Output = AnyType;

    fn bitor(self, rhs: Self) -> Self::Output {
        AnyType::Or(Box::new(self), Box::new(rhs))
    }
}

impl BitAnd for AnyType {
    type Output = AnyType;

    fn bitand(self, rhs: Self) -> Self::Output {
        AnyType::And(Box::new(self), Box::new(rhs))
    }
}

impl Not for AnyType {
    type Output = Self;

    fn not(self) -> Self::Output {
        AnyType::Not(Box::new(self))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum InputChar {
    // 在行首时使用，标记为行首
    // 在上一个状态为\n 符的终结状态，返回的下一状态初始即为行首状态
    LineStart,
    // 普通输入状态
    // 包含一个普通字符
    Char(char),
    // 读取结束 当读使用end_input 时返回的input
    Eof,
    Any(AnyType),
}

impl Default for InputChar {
    fn default() -> Self {
        Self::LineStart
    }
}
impl Into<Option<char>> for &InputChar {
    fn into(self) -> Option<char> {
        match self {
            InputChar::Char(c) => Some(*c),
            _ => None,
        }
    }
}
impl From<char> for InputChar {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

pub enum NextStatus {
    /// goon 状态机还能进行下一步转换，继续转换，携带下一状态
    GoOn(Status),
    /// accept 状态机无法继续转换，且处于接受状态，返回接受对应的Token
    Final(
        /// 接受状态对应的token
        Box<dyn FullToken>,
        //原始数据
        Vec<char>,
        /// 自动机重启后的开始状态
        Status,
        /// 自动机重启后的第一个输入
        InputChar,
    ),
    // raw input 缓冲区数据，当自动机无法继续转换下去且不处于接受状态，返回Buff数据
    Plain(
        // 对应的buff数据
        Vec<char>,
        // 重启自动机后的开始状态
        Status,
        // 重启后第一次输入的字符
        InputChar,
    ),
}

#[derive(Debug,PartialEq, Eq)]
pub (super)enum LineStatus {
    Normal,
    LineStart,
}

impl LineStatus  {
    pub(super) fn update(&mut self,input:&InputChar){
        if let InputChar::Char(c)=input{
            if c==&'\n'{
                *self=Self::LineStart
            }else if !c.is_whitespace(){
                *self=Self::Normal
            }
        }else if let InputChar::LineStart=input{
            *self=LineStatus::LineStart
        }else if let InputChar::Any(AnyType::Char('\n'))=input {
            *self=LineStatus::LineStart
        }
    }
}

#[cfg(test)]
mod test{
    use super::LineStatus;

    #[test]
    fn test_line_status() {
        let mut init=LineStatus::LineStart;
        init.update(&'a'.into());
        assert_eq!(init,LineStatus::Normal);
        init.update(&'\n'.into());
        assert_eq!(init,LineStatus::LineStart);
        init.update(&' '.into());
        assert_eq!(init,LineStatus::LineStart);
        init.update(&'\t'.into());
        assert_eq!(init,LineStatus::LineStart);

        init.update(&'k'.into());
        assert_eq!(init,LineStatus::Normal);
    }
}