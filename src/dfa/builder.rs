use crate::tokens::{Title1, TripleStar};

use self::trans_holder::TransHolder;

use super::{
    core::DFA,
    utils::AdHocCanAny,
    wraps::{AnyType, InputChar, Status},
};
use crate::tokens::{
    BoxEnd, BoxMid, ChangeLine, DoubleStar, Idented, ImgStart, LinkStart, NewParam, OrderList,
    Reference, SepChar, SepLine, Star, Title2, Title3, Title4, Title5, Title6, UnorderList,
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
        let mut holder = TransHolder::new(self.init_count+1);

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
                h.set_accept_status(next, LinkStart);
                //image start
                next = h.add_tran_with_auto_next(0, '!');
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
                h.add_can_any(
                    next,
                    AdHocCanAny::new(|c| match c.is_digit(10) {
                        true => Some(AnyType::Digit),
                        false => None,
                    }),
                );
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
                next = h.add_tran_with_auto_next(next, '*');
                h.set_accept_status(next, DoubleStar);
                next = h.add_tran_with_auto_next(next, '*');
                h.set_accept_status(next, TripleStar);

                //specal type
                //sep
                next = h.add_tran_with_auto_next(0, ' ');
                h.set_accept_status(next, SepChar);
                // // line change
                next = h.add_tran_with_auto_next(0, '\n');
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
            })
            .build()
    }
}

#[cfg(test)]
mod test {

    use crate::dfa::{
        interface::{FullToken, TokenTrait},
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
        let mut dfa = DFABuilder::new(0, 1)
            .add_trans(|h| {
                let i = ' ';
                let mut next = h.add_tran_with_auto_next(1, i);
                next = h.add_tran_with_auto_next(next, i);
                next = h.add_tran_with_auto_next(next, i);
                next = h.add_tran_with_auto_next(next, i);
                next = h.add_tran(1, '\t', next);

                h.set_accept_status(next, Indented)
            })
            .build();
        let mut init = dfa.init();
        let mut last = None;

        println!("init {:?}", &init);

        let mut iter = vec!['\t', ' ', ' ', ' ', 'a'].into_iter();
        loop {
            let input = if let Some(c) = last {
                last = None;
                c
            } else if let Some(input) = iter.next() {
                input
            } else {
                break;
            };

            println!(" input {:?}", &input);
            match dfa.next_status(init, input.into()) {
                NextStatus::GoOn(go) => {
                    println!("next {:?}", &go);
                    init = go;
                }
                NextStatus::Final(r, ns, i) => {
                    assert_eq!(i, InputChar::Char(' '));
                    assert_eq!(ns, Status(1));
                    assert_eq!(init, Status(5));
                    if let InputChar::Char(c) = i {
                        last = Some(c);
                    }
                    println!("{:?}", ns);
                    init = ns;
                }
                NextStatus::Plain(p, s, i) => {
                    assert_eq!(input, 'a');

                    assert_eq!(p, vec![' ', ' ', ' ']);
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

    #[test]
    fn test_full_dfa() {
        let dfa=DFABuilder::init();

        print!("{}",dfa);
    }
}
