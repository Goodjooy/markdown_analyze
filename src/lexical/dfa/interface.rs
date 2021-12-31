use super::wraps::AnyType;

pub trait CanAny {
    fn can_any(&self, input: char) -> Option<AnyType>;
}


//TODO impl Token Serder
