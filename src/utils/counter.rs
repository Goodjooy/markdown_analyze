use std::ops::Add;

pub(crate) struct Counter(usize);

impl Counter {
    pub(crate) fn new() -> Counter {
        Self(0)
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let d = self.0;
        self.0 += 1;
        Some(d)
    }
}
