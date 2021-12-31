use std::ops::BitOr;

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
    // 其他类型（要提供标记id）
    Orther(usize),
    // 混合
    Conbin(Box<AnyType>, Box<AnyType>),
}

impl Into<InputChar> for AnyType {
    fn into(self) -> InputChar {
        InputChar::Any(self)
    }
}

impl BitOr for AnyType {
    type Output = AnyType;

    fn bitor(self, rhs: Self) -> Self::Output {
        AnyType::Conbin(Box::new(self), Box::new(rhs))
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



