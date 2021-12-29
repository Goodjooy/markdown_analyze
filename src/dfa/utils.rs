use super::{interface::CanAny, wraps::AnyType};

pub struct AdHocCanAny<F: Fn(char) -> Option<AnyType>> {
    handle: F,
}

impl<F> AdHocCanAny<F>
where
    F: Fn(char) -> Option<AnyType>,
    F: 'static,
{
    pub fn new(handle: F) -> Box<dyn CanAny> {
        let v = Self { handle };
        Box::new(v)
    }
}

impl<F> CanAny for AdHocCanAny<F>
where
    F: Fn(char) -> Option<AnyType>,
{
    fn can_any(&self, input: char) -> Option<AnyType> {
        let handle = &self.handle;
        handle(input)
    }
}
