use std::{collections::HashMap, usize};

use crate::{
    dfa::{
        interface::{CanAny, TokenTrait},
        wraps::{AnyType, InputChar},
    },
    utils::counter::Counter,
};

pub struct TransHolder {
    pub(super) trans: HashMap<(usize, InputChar), usize>,
    pub(super) any_trans: HashMap<usize, Box<dyn CanAny>>,
    pub(super) ac_status: HashMap<usize, Box<dyn TokenTrait>>,
    counter: Counter,
}

impl TransHolder {
    pub(super) fn new(init: usize) -> Self {
        Self {
            trans: HashMap::with_capacity(16),
            any_trans: HashMap::with_capacity(16),
            ac_status: HashMap::with_capacity(16),
            counter: Counter::init(init),
        }
    }

    pub(super) fn get_counter(&self) -> usize {
        self.counter.0
    }
}

impl TransHolder {
    pub fn add_normal_tran(&mut self, src: usize, input: char, dst: usize) -> usize {
        self.trans.insert((src, input.into()), dst);
        dst
    }

    pub fn add_normal_tran_with_auto_next(&mut self, src: usize, input: char) -> usize {
        let next = self.counter.next().unwrap();
        self.add_normal_tran(src, input, next)
    }

    pub fn add_any_tran(&mut self, src: usize, input: AnyType, dst: usize) -> usize {
        self.trans.insert((src, input.into()), dst);
        dst
    }

    pub fn add_any_tran_with_auto_next(&mut self, src: usize, input: AnyType) -> usize {
        let dst = self.counter.next().unwrap();
        self.add_any_tran(src, input, dst)
    }

    pub fn add_can_any(&mut self, src: usize, any_tran: Box<dyn CanAny>) {
        self.any_trans.insert(src, any_tran);
    }

    pub fn set_accept_status<T: Sized + TokenTrait + 'static>(&mut self, status: usize, result: T) {
        let wrap: Box<dyn TokenTrait> = Box::new(result);
        self.ac_status.insert(status, wrap);
    }
}
