use super::interface::FullToken;

#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Status(pub(super) usize);




impl From<usize> for Status {
    fn from(i: usize) -> Self {
        Self(i)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AnyType {
    Digit,
    WideSpace,

    Conbin(Box<AnyType>, Box<AnyType>),
}

impl Into<InputChar> for AnyType {
    fn into(self) -> InputChar {
        InputChar::Any(self)
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
    GoOn(Status),
    Final(Box<dyn FullToken>, Status, InputChar),
    Plain(Vec<char>, Status, InputChar),
}