use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use serde::de::IntoDeserializer;

use crate::tokens::Title1;

use self::trans_holder::TransHolder;

use super::{
    core::DFA,
    interface::{CanAny, TokenTrait},
    utils::AdHocCanAny,
    wraps::{AnyType, InputChar, Status},
};
use crate::tokens::{
    BoxEnd, BoxMid, ChangeLine, DoubleStar, Idented, ImgStart, LinkStart, NewParam, OrderList,
    Reference, SepChar, SepLine, Star, Title2, Title3, Title4, Title5, Title6, TribleStar,
    UnorderList,
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
        let mut holder = TransHolder::new(self.init_count);

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
                let mut next = h.add_normal_tran_with_auto_next(1, c);
                h.set_accept_status(next, Title1);
                //##
                next = h.add_normal_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title2);
                //###
                next = h.add_normal_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title3);
                //####
                next = h.add_normal_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title4);
                //#####
                next = h.add_normal_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title5);
                //######
                next = h.add_normal_tran_with_auto_next(next, c);
                h.set_accept_status(next, Title6);
                // //refer
                // t.insert(('>'.into(), 1), 8);
                // t.insert(('>'.into(), 8), 8);
                // f.insert(8, Rc::new(Reference));
                // //link start
                // t.insert(('['.into(), 0), 9);
                // f.insert(9, Rc::new(LinkStart));
                // //image start
                // t.insert(('!'.into(), 0), 10);
                // t.insert(('['.into(), 10), 11);
                // f.insert(11, Rc::new(ImgStart));
                // //box mid
                // t.insert((']'.into(), 0), 12);
                // t.insert(('('.into(), 12), 13);
                // f.insert(13, Rc::new(BoxMid));
                // // box mid
                // t.insert((')'.into(), 0), 14);
                // f.insert(14, Rc::new(BoxEnd));
                // // unorder list
                // t.insert(('-'.into(), 1), 15);
                // f.insert(15, Rc::new(UnorderList));
                // t.insert(('*'.into(), 1), 16);
                // f.insert(16, Rc::new(UnorderList));
                // // order list
                h.add_can_any(
                    next,
                    AdHocCanAny::new(|c| match c.is_digit(10) {
                        true => Some(AnyType::Digit),
                        false => None,
                    }),
                );
                next = h.add_any_tran_with_auto_next(next, AnyType::Digit);
                next = h.add_normal_tran_with_auto_next(next, '.');
                // t.insert(('0'.into(), 1), 17);
                // t.insert(('0'.into(), 17), 17);
                // t.insert(('1'.into(), 1), 17);
                // t.insert(('1'.into(), 17), 17);
                // t.insert(('2'.into(), 1), 17);
                // t.insert(('2'.into(), 17), 17);
                // t.insert(('3'.into(), 1), 17);
                // t.insert(('3'.into(), 17), 17);
                // t.insert(('4'.into(), 1), 17);
                // t.insert(('4'.into(), 17), 17);
                // t.insert(('5'.into(), 1), 17);
                // t.insert(('5'.into(), 17), 17);
                // t.insert(('6'.into(), 1), 17);
                // t.insert(('6'.into(), 17), 17);
                // t.insert(('7'.into(), 1), 17);
                // t.insert(('7'.into(), 17), 17);
                // t.insert(('8'.into(), 1), 17);
                // t.insert(('8'.into(), 17), 17);
                // t.insert(('9'.into(), 1), 17);
                // t.insert(('9'.into(), 17), 17);
                // t.insert(('.'.into(), 17), 18);
                // f.insert(18, Rc::new(OrderList));
                // // sep line
                // t.insert(('-'.into(), 15), 19);
                // t.insert(('-'.into(), 19), 20);
                // f.insert(20, Rc::new(SepLine));
                // t.insert(('*'.into(), 16), 21);
                // t.insert(('*'.into(), 21), 22);
                // f.insert(22, Rc::new(SepLine));
                // //star
                // t.insert(('*'.into(), 0), 23);
                // f.insert(23, Rc::new(Star));
                // t.insert(('*'.into(), 23), 24);
                // f.insert(23, Rc::new(DoubleStar));
                // t.insert(('*'.into(), 24), 25);
                // f.insert(23, Rc::new(TribleStar));

                // //specal type
                // //sep
                // t.insert((' '.into(), 0), 24);
                // f.insert(24, Rc::new(SepChar));
                // // line change
                // t.insert(('\n'.into(), 0), 25);
                // f.insert(25, Rc::new(ChangeLine));
                // // parghe change
                // t.insert((' '.into(), 24), 26);
                // t.insert(('\n'.into(), 26), 27);
                // f.insert(27, Rc::new(NewParam));
                // //idented
                // t.insert((' '.into(), 1), 28);
                // t.insert((' '.into(), 28), 29);
                // t.insert((' '.into(), 29), 30);
                // t.insert((' '.into(), 30), 31);
                // t.insert(('\t'.into(), 1), 31);
                // f.insert(31, Rc::new(Idented));

                // t.insert((InputChar::Any(AnyType::Digit), 31), 32);
            })
            .build()
    }
}

#[cfg(test)]
mod test {

    use crate::dfa::{
        interface::FullToken,
        wraps::{NextStatus, Status},
    };

    use super::*;

    #[derive(Debug, Clone)]
    struct Indented;
    impl FullToken for Indented {}

    impl TokenTrait for Indented {
        fn name(&self) -> &'static str {
            "Idented"
        }

        fn to_full(&self, _: &[char]) -> Box<dyn FullToken> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn test_builder() {
        let i: Rc<dyn TokenTrait> = Rc::new(Indented);
        let mut dfa = DFABuilder::new(0, 1)
            .add_trans(|h| {
                let i = ' ';
                let mut next = h.add_normal_tran_with_auto_next(1, i);
                next = h.add_normal_tran_with_auto_next(next, i);
                next = h.add_normal_tran_with_auto_next(next, i);
                next = h.add_normal_tran_with_auto_next(next, i);
                next = h.add_normal_tran(1, '\t', next);

                h.set_accept_status(next, Indented)
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
                    //assert_eq!("Idented", r.name());
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
