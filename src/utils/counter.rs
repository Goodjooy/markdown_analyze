pub(crate) struct Counter(pub(crate) usize);

impl Counter {
    pub(crate) fn new() -> Counter {
        Self(0)
    }
    pub(crate) fn init(init: usize) -> Counter {
        Self(init)
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
