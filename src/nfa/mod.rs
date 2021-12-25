use std::{
    collections::{HashMap, HashSet},
    ops::{BitAnd, BitOr},
};

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
