use super::super::token_trait::{FullToken, TokenTrait};

#[macro_export]
macro_rules! title_token_gen {
    ($l:literal,$n:ident,$name:literal) => {
        pub struct $n;

        impl TokenTrait for $n {
            fn name(&self) -> &'static str {
                $name
            }
            fn to_full(&self, _buff: &[char]) -> Box<dyn FullToken> {
                Box::new(TitleToken { level: $l })
            }
        }
    };
}

pub struct TitleToken {
    pub level: u8,
}

impl FullToken for TitleToken {}

title_token_gen!(1,Title1,"h1");
title_token_gen!(2,Title2,"h2");
title_token_gen!(3,Title3,"h3");
title_token_gen!(4,Title4,"h4");
title_token_gen!(5,Title5,"h5");
title_token_gen!(6,Title6,"h6");