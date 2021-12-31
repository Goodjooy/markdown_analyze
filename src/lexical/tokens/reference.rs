use full_token_derive_macro::FullToken;

use super::super::token_trait::{TokenTrait, FullToken};


pub struct PartRef;

impl TokenTrait for PartRef {
    fn name(&self) -> &'static str {
        "refer"
    }

    fn to_full(&self, buff: &[char]) -> Box<dyn FullToken> {
        Box::new(Reference{depath:buff.len() as u64})
    }
}

#[derive(FullToken)]
#[token(name="refer")]
pub struct Reference{
    pub depath:u64
}

