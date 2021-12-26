use std::{
    collections::{HashMap, HashSet, LinkedList},
    ops::{BitAnd, BitOr}, rc::Rc,
};

use crate::{ utils::counter::Counter, dfa::core::DFA};

mod and_nfa;
mod or_nfa;
#[derive(Debug)]
pub struct MacroNfa {
    trans_table: HashMap<(usize, char), usize>,
    none_trans_table: HashMap<usize, Vec<usize>>,
    status_set: HashSet<usize>,
    input_set: HashSet<char>,
    start: usize,
    final_status: usize,

}

impl MacroNfa {
    pub fn new(start: usize, final_status: usize) -> Self {
        Self {
            trans_table: HashMap::with_capacity(16),
            none_trans_table: HashMap::with_capacity(16),
            start,
            final_status,
            status_set: HashSet::with_capacity(16),
            input_set: HashSet::with_capacity(16),
        }
    }

    pub fn add_trans(&mut self, (srcs, input): (usize, Option<char>), dst: usize) {
        self.status_set.insert(srcs);
        self.status_set.insert(dst);
        if let Some(i) = input {
            self.input_set.insert(i);
            self.trans_table.insert((srcs, i), dst);
        } else {
            if let Some(v) = self.none_trans_table.get_mut(&srcs) {
                v.push(dst)
            } else {
                self.none_trans_table.insert(srcs, vec![dst]);
            }
        }
    }
    fn max_status_id(&self, rhs: &Self) -> usize {
        let counter = *self
            .status_set
            .iter()
            .max()
            .unwrap_or(&0)
            .max(rhs.status_set.iter().max().unwrap_or(&0))
            + 1;
        counter
    }

    fn generate_conflict_map(&self, rhs: &Self) -> HashMap<usize, usize> {
        let mut counter = self.max_status_id(rhs);
        let status_map = rhs
            .status_set
            .intersection(&self.status_set)
            .into_iter()
            .map(|sr| {
                let t = (*sr, counter);
                counter += 1;
                t
            })
            .collect::<HashMap<_, _>>();
        status_map
    }

    fn conbin_trans(&mut self, rhs: &Self, status_map: &HashMap<usize, usize>) {
        // 将rhs全部状态添加到self中
        for ((s, i), d) in rhs.trans_table.iter().map(|((src, inp), dst)| {
            let src = into_no_conflict(src, &status_map);
            let dst = into_no_conflict(dst, &status_map);
            ((src, *inp), dst)
        }) {
            self.add_trans((s, Some(i)), d)
        }

        for (k, v) in rhs.none_trans_table.iter().map(|(k, v)| {
            let k = into_no_conflict(k, &status_map);
            let v = v
                .iter()
                .map(|v| into_no_conflict(v, &status_map))
                .collect::<Vec<_>>();
            (k, v)
        }) {
            for vd in v {
                self.add_trans((k, None), vd);
            }
        }
    }
}
fn into_no_conflict(src: &usize, conflict_map: &HashMap<usize, usize>) -> usize {
    if let Some(s) = conflict_map.get(src) {
        *s
    } else {
        *src
    }
}

impl BitAnd for MacroNfa {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        let conflict_map = self.generate_conflict_map(&rhs);
        self.conbin_trans(&rhs, &conflict_map);
        // 连接状态机
        self.add_trans(
            (self.final_status, None),
            into_no_conflict(&rhs.start, &conflict_map),
        );
        // 将终止状态更新
        self.final_status = into_no_conflict(&rhs.final_status, &conflict_map);
        self.status_set.insert(self.final_status);

        Self { ..self }
    }
}

impl BitOr for MacroNfa {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        let conflict_map = self.generate_conflict_map(&rhs);

        self.conbin_trans(&rhs, &conflict_map);
        let mut counter = self.max_status_id(&rhs);

        self.add_trans((counter, None), self.start);
        self.add_trans((counter, None), into_no_conflict(&rhs.start, &conflict_map));
        self.start = counter;

        counter += 1;
        self.add_trans((self.final_status, None), counter);
        self.add_trans(
            (into_no_conflict(&rhs.final_status, &conflict_map), None),
            counter,
        );
        self.final_status = counter;

        Self { ..self }
    }
}

impl MacroNfa {
    fn extand_set(&self, tgt: &mut HashSet<usize>) {
        let mut queue = LinkedList::<usize>::new();
        for s in tgt.iter() {
            queue.push_back(*s);
        }
        queue.push_back(self.start);
        while let Some(ne) = queue.pop_front() {
            tgt.insert(ne);
            if let Some(f) = self.none_trans_table.get(&ne) {
                for v in f {
                    queue.push_back(*v)
                }
            }
        }
    }
}
// dfa 状态
#[derive(Eq)]
struct DState {
    nfas: Vec<usize>,
}

impl PartialEq for DState {
    fn eq(&self, other: &Self) -> bool {
        self.nfas == other.nfas
    }
}

impl DState {
    fn new(set: &HashSet<usize>) -> Self {
        Self {
            nfas: set.iter().map(|v| *v).collect(),
        }
    }
}

impl std::hash::Hash for DState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nfas.hash(state);
    }
}

impl Into<DFA> for MacroNfa {
    fn into(self) -> DFA {
        // 全部DFA状态
        let mut d_states = HashMap::<DState, usize>::new();
        // 未标记的状态
        let mut d_unsigned_states = LinkedList::<HashSet<usize>>::new();
        let mut d_trans = HashMap::<(usize, char), usize>::new();
        let mut counter = Counter::new();
        // from start status
        // collect all reachable status by input nil
        let mut status = HashSet::<usize>::from_iter([self.start]);
        let mut queue = LinkedList::<usize>::new();
        self.extand_set(&mut status);
        // 添加初始状态

        let start_id = counter.next().unwrap();
        let mut final_id = Option::<usize>::None;

        d_states.insert(DState::new(&status), start_id);
        d_unsigned_states.push_back(status);

        while let Some(state) = d_unsigned_states.pop_front() {
            let src_state = DState::new(&state);
            // 对于每个未标记的状态，必定是一个DFA 状态
            let src_id = *d_states.get(&src_state).unwrap();

            //循环输入符号表，
            for sign in self.input_set.iter() {
                let mut res = state
                    .iter()
                    .map(|v| *v)
                    .filter_map(|v| self.trans_table.get(&(v, *sign)))
                    .map(|f| *f)
                    .collect::<HashSet<_>>();
                self.extand_set(&mut res);

                let status = DState::new(&res);
                let set_final = res.contains(&self.final_status);

                let id = if let Some(i) = d_states.get(&status) {
                    *i
                } else {
                    // un regeisted status
                    let id = counter.next().unwrap();
                    d_states.insert(status, id);
                    d_unsigned_states.push_back(res);

                    id
                };
                if set_final {
                    final_id = Some(id);
                }
                //add trans
                d_trans.insert((src_id, *sign), id);
            }
        }

      
        
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitand() {
        let mut a = MacroNfa::new(0, 1);
        a.add_trans((0, Some(' ')), 2);
        a.add_trans((2, Some(' ')), 1);

        let mut b = MacroNfa::new(0, 1);
        b.add_trans((0, Some('a')), 2);
        b.add_trans((2, Some('b')), 1);

        let c = a & b;

        println!("{:#?}", c);
    }

    #[test]
    fn test_bitor() {
        let mut a = MacroNfa::new(0, 1);
        a.add_trans((0, Some(' ')), 2);
        a.add_trans((2, Some(' ')), 1);

        let mut b = MacroNfa::new(0, 1);
        b.add_trans((0, Some('b')), 2);
        b.add_trans((2, Some('a')), 1);

        let e = a | (b);

        println!("{:#?}", e);
    }

    #[test]
    fn test_conbin() {
        let mut a = MacroNfa::new(0, 1);
        a.add_trans((0, Some(' ')), 2);
        a.add_trans((2, Some(' ')), 1);

        let mut b = MacroNfa::new(0, 1);
        b.add_trans((0, Some('b')), 2);
        b.add_trans((2, Some('a')), 1);

        let mut c = MacroNfa::new(0, 1);
        c.add_trans((0, Some('#')), 2);
        c.add_trans((2, Some('#')), 1);

        let t = b & c;
        let e = a | (t);

        println!("{:#?}", e);
    }
}
