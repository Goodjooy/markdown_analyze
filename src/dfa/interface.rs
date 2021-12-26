use super::wraps::AnyType;

pub trait CanAny {
    fn can_any(&self, input: char) -> Option<AnyType>;
}

pub trait TokenTrait {
    fn name(&self) -> &'static str;
    fn to_full(&self, buff: &[char]) -> Box<dyn FullToken>;
}

pub trait FullToken {

}
