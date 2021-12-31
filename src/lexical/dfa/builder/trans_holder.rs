use super::super::{interface::CanAny, wraps::InputChar};
use std::{collections::HashMap, usize};

use crate::{lexical::token_trait::TokenTrait, utils::counter::Counter};

pub struct TransHolder {
    pub(super) trans: HashMap<(usize, InputChar), usize>,
    pub(super) any_trans: HashMap<usize, Box<dyn CanAny>>,
    pub(super) ac_status: HashMap<usize, Box<dyn TokenTrait>>,
    line_start: usize,
    init_status: usize,
    counter: Counter,
}

impl TransHolder {
    pub(super) fn new(init: usize, line_start: usize, init_status: usize) -> Self {
        Self {
            trans: HashMap::with_capacity(16),
            any_trans: HashMap::with_capacity(16),
            ac_status: HashMap::with_capacity(16),
            counter: Counter::init(init),
            line_start,
            init_status,
        }
    }

    pub(super) fn get_counter(&self) -> usize {
        self.counter.0
    }
}

impl TransHolder {
    pub fn add_tran<T>(&mut self, src: usize, input: T, dst: usize) -> usize
    where
        T: Into<InputChar> + Clone,
    {
        // if trans exist ,do not change
        if let Some(dst) = self.trans.get(&(src, input.clone().into())) {
            *dst
        } else {
            self.trans.insert((src, input.into()), dst);
            dst
        }
    }

    pub fn add_tran_with_auto_next<T>(&mut self, src: usize, input: T) -> usize
    where
        T: Into<InputChar> + Clone,
    {
        let next = self.counter.next().unwrap();
        self.add_tran(src, input, next)
    }

    pub fn add_can_any(&mut self, src: usize, any_tran: Box<dyn CanAny>) {
        self.any_trans.insert(src, any_tran);
    }

    pub fn set_accept_status<T: Sized + TokenTrait + 'static>(&mut self, status: usize, result: T) {
        let wrap: Box<dyn TokenTrait> = Box::new(result);
        self.ac_status.insert(status, wrap);
    }
}
