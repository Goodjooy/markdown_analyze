//! 状态机，用于文法分析
//!

use std::{collections::HashMap, rc::Rc};

#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Status(usize);

impl From<usize> for Status {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum InputChar {
    // 在行首时使用，标记为行首
    // 在上一个状态为\n 符的终结状态，返回的下一状态初始即为行首状态
    LineStart,
    // 普通输入状态
    // 包含一个普通字符
    Char(char),
    // 读取结束 当读使用end_input 时返回的input
    Eof,
}

impl Default for InputChar {
    fn default() -> Self {
        Self::LineStart
    }
}

impl From<char> for InputChar {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

pub trait TokenTrait {
    fn name(&self) -> &'static str;
}

pub enum NextStatus {
    GoOn(Status),
    Final(Rc<dyn TokenTrait>, Status, InputChar),
    Plain(Vec<InputChar>, Status, InputChar),
}

pub struct DFA {
    // 状态钻换表
    table: HashMap<(InputChar, Status), Status>,
    // 初始状态
    init_status: Status,
    // 行首状态
    line_start: Status,
    // 终结状态
    // 在下一个输入进入后无法继续转换或者输入结束
    final_status: HashMap<Status, Rc<dyn TokenTrait>>,
    //input buffer
    //如果读取到行末却没有对应终结状态，就将buffer内打包为普通文本处理
    buff: Vec<InputChar>,
}

impl DFA {
    fn new() -> Self {
        Self {
            table: HashMap::with_capacity(32),
            init_status: Status(0),
            line_start: Status(1),
            final_status: HashMap::with_capacity(32),
            buff: Vec::with_capacity(512),
        }
    }

    pub fn next_status(&mut self, status: Status, input: InputChar) -> NextStatus {
        // 自动机可以继续转换下去，继续转换
        if let Some(ns) = self.table.get(&(input, status)) {
            self.buff.push(input);
            NextStatus::GoOn(*ns)
        }
        // 自动机无法继续转换下去，但是处于终结状态, 返回终结token并且返回下一状态和input
        else if let Some(fin) = self.final_status.get(&status) {
            let sta = self.reset_status();
            self.buff.clear();
            NextStatus::Final(Rc::clone(fin), sta, input)
        }
        // 自动机无法继续转换下去, 且处于非终结状态，恐慌模式，返回buff，重置状态
        else {
            let sta = self.reset_status();
            NextStatus::Plain(self.buff.clone(), sta, input)
        }
    }

    pub fn input_end(&mut self) -> NextStatus {
        let sta = self.reset_status();
        NextStatus::Plain(self.buff.clone(), sta, InputChar::Eof)
    }

    fn reset_status(&self) -> Status {
        if let Some(last) = self.buff.last() {
            if last == &InputChar::Char('\n') {
                return self.line_start.clone();
            }
        }
        return self.init_status.clone();
    }

    pub fn init(&self) -> Status {
        return self.line_start;
    }
}

pub struct DFABuilder {
    inner: DFA,
    set_init: bool,
    set_start: bool,
}

impl DFABuilder {
    pub fn new(init_status: usize, line_start: usize) -> Self {
        Self {
            inner: DFA::new(),
            set_init: false,
            set_start: false,
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
        F: FnOnce(&mut HashMap<(char, usize), usize>, &mut HashMap<usize, Rc<dyn TokenTrait>>),
    {
        let mut table = HashMap::with_capacity(16);
        let mut final_status = HashMap::with_capacity(16);

        handle(&mut table, &mut final_status);

        let table = table
            .into_iter()
            .map(|((c, s), so)| ((InputChar::Char(c), Status(s)), Status(so)));

        let final_status = final_status.into_iter().map(|(k, v)| (Status(k), v));

        self.inner.table.extend(table);
        self.inner.final_status.extend(final_status);

        self
    }

    pub fn build(self) -> DFA {
        self.inner
    }
}

#[cfg(test)]
mod test {

    use super::*;

    struct Indented;
    impl TokenTrait for Indented {
        fn name(&self) -> &'static str {
            "Idented"
        }
    }

    #[test]
    fn test_builder() {
        let i: Rc<dyn TokenTrait> = Rc::new(Indented);
        let mut dfa = DFABuilder::new(0, 1)
            .add_trans(|t, f| {
                t.insert((' ', 1), 2);
                t.insert((' ', 2), 3);
                t.insert((' ', 3), 4);
                t.insert((' ', 4), 5);
                t.insert(('\t', 1), 5);

                f.insert(5, Rc::clone(&i));
            })
            .build();
        let mut init = dfa.init();
        let mut last = None;

        println!("{:?}", &init);

        let mut iter = vec![' ', ' ', ' ', ' ', 'a'].into_iter();
        loop {
            let input = if let Some(c) = last {
                last = None;
                c
            } else if let Some(input) = iter.next() {
                input
            } else {
                break;
            };

            println!("{:?}", &input);
            match dfa.next_status(init, input.into()) {
                NextStatus::GoOn(go) => {
                    println!("{:?}", &go);
                    init = go;
                }
                NextStatus::Final(r, ns, i) => {
                    assert_eq!("Idented", r.name());
                    assert_eq!(i, InputChar::Char('a'));
                    assert_eq!(ns, Status(0));
                    assert_eq!(init, Status(5));
                    if let InputChar::Char(c) = i {
                        last = Some(c);
                    }
                    println!("{:?}", ns);
                    init = ns;
                }
                NextStatus::Plain(p, s, i) => {
                    assert_eq!(input, 'a');

                    assert_eq!(p, vec![]);
                    assert_eq!(s, Status(0));
                    assert_eq!(i, InputChar::Char('a'));
                }
            }
        }

        let end = dfa.input_end();
        if let NextStatus::Plain(p, s, i) = end {
            assert_eq!(p, vec![]);
            assert_eq!(s, Status(0));
            assert_eq!(i, InputChar::Eof);
        } else {
            unreachable!()
        }
    }
}
