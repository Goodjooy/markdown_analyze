use std::collections::HashMap;

use crate::lexical::token_trait::TokenTrait;

use super::{
    interface::{CanAny},
    wraps::{InputChar, NextStatus, Status},
};

pub struct DFA {
    // 状态钻换表
    pub(super) table: HashMap<(InputChar, Status), Status>,
    // 初始状态
    pub(super) init_status: Status,
    // 行首状态
    pub(super) line_start: Status,
    // 终结状态
    // 在下一个输入进入后无法继续转换或者输入结束
    pub(super) final_status: HashMap<Status, Box<dyn TokenTrait>>,
    // any trans
    pub(super) any_trans: HashMap<Status, Box<dyn CanAny>>,
    //input buffer
    //如果读取到行末却没有对应终结状态，就将buffer内打包为普通文本处理
    pub(super) buff: Vec<char>,
    full_buff:Vec<char>,
}

impl DFA {
    pub(super) fn new() -> Self {
        Self {
            table: HashMap::with_capacity(32),
            init_status: Status(0),
            line_start: Status(1),
            final_status: HashMap::with_capacity(32),
            buff: Vec::with_capacity(16),
            full_buff:Vec::with_capacity(512),
            any_trans: HashMap::with_capacity(32),
        }
    }
    /// 状态转换函数  
    ///
    /// ---
    ///
    /// todo 写注释
    pub fn next_status(&mut self, status: Status, input: InputChar) -> NextStatus {
        // 自动机可以继续转换下去，继续转换
        if let Some(ns) = self
            .table
            .get(&(input.clone(), status))
            .or(Into::<Option<char>>::into(&input)
                .and_then(|c: char| self.any_trans.get(&status).and_then(|ca| ca.can_any(c)))
                .and_then(|at| self.table.get(&(InputChar::Any(at), status))))
        {
            if let InputChar::Char(c) = input {
                self.buff.push(c);
                self.full_buff.push(c);
            }
            NextStatus::GoOn(*ns)
        }
        // 自动机无法继续转换下去，但是处于终结状态, 返回终结token并且返回下一状态和input
        else if let Some(fin) = self.final_status.get(&status) {
            let sta = self.reset_status();
            let data = fin.to_full(self.buff.as_slice());
            let res = NextStatus::Final(data, self.buff.clone(), sta, input);
            self.buff.clear();
            res
        }
        // 自动机无法继续转换下去, 且处于非终结状态，恐慌模式，返回buff，重置状态
        else {
            
            let sta = 
            // 如果缓冲区为空，当前输入字符非换行就是非行首
            if self.buff.len()==0 {
                if let InputChar::Char(c)=input{
                    
                    self.full_buff.push(c);
                }
                self.init_status
            }else{
                self.reset_status()
            };
            NextStatus::Plain(self.buff.clone(), sta, input)
        }
    }

    pub fn input_end(&mut self) -> NextStatus {
        let sta = self.reset_status();
        NextStatus::Plain(self.buff.clone(), sta, InputChar::Eof)
    }

    fn reset_status(&self) -> Status {
        let mut idx = self.full_buff.len();
        // check empty buff
        if idx == 0 {
            return self.line_start;
        } else {
            idx -= 1;
        }

        // 循环跳过空格到第一个不是空格的地方
        while unsafe { self.full_buff.get_unchecked(idx) }.is_whitespace()
            && unsafe { self.full_buff.get_unchecked(idx) } != &'\n'
        {
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
        // check last non empty char
        if unsafe { self.full_buff.get_unchecked(idx) } == &'\n' || idx == 0 {
            self.line_start
        } else {
            self.init_status.clone()
        }
    }

    pub fn init(&self) -> Status {
        self.reset_status()
    }

    pub fn reset(&mut self){
        self.buff.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reset_status() {
        let mut dfa = DFA::new();
        // read a Idented  still in the line start
        dfa.buff.extend(['a', '\n', ' ', ' ', ' ', ' ']);
        let res = dfa.reset_status();
        dfa.buff.clear();
        assert_eq!(res, dfa.line_start);

        // read a Ident \t is line start
        dfa.buff.extend(['a', '\n', '\t']);
        let res = dfa.reset_status();
        dfa.buff.clear();
        assert_eq!(res, dfa.line_start);

        // read 2 mixed ident is line start
        dfa.buff.extend(['a', '\n', '\t', ' ', ' ', ' ', ' ']);
        let res = dfa.reset_status();
        dfa.buff.clear();
        assert_eq!(res, dfa.line_start);

        dfa.buff.extend(['a', '\n', ' ', ' ', ' ', ' ', '\t']);
        let res = dfa.reset_status();
        dfa.buff.clear();
        assert_eq!(res, dfa.line_start);

        // read title sign ,not a line start
        dfa.buff.extend(['a', '\n', '#', ' ']);
        let res = dfa.reset_status();
        dfa.buff.clear();
        assert_eq!(res, dfa.init_status);

        // end with \n is line start
        dfa.buff.extend(['a', ' ', ' ', '\n']);
        let res = dfa.reset_status();
        dfa.buff.clear();
        assert_eq!(res, dfa.line_start);

        // empty buff is line start
        dfa.buff.clear();
        let res = dfa.reset_status();
        assert_eq!(res, dfa.line_start);
    }
}
