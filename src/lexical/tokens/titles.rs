use full_token_derive_macro::FullToken;

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

#[derive(FullToken)]
#[token(name = "title")]
pub struct TitleToken {
    pub level: u8,
}

title_token_gen!(1, Title1, "h1");
title_token_gen!(2, Title2, "h2");
title_token_gen!(3, Title3, "h3");
title_token_gen!(4, Title4, "h4");
title_token_gen!(5, Title5, "h5");
title_token_gen!(6, Title6, "h6");

#[cfg(test)]
mod test {

    use full_token_derive_macro::FullToken;

    use crate::lexical::token_trait::{FromToken, FullToken};

    #[derive(FullToken, PartialEq, Eq, Debug)]
    #[token(name = "mcok")]
    pub struct MockToken {
        #[token(name = "mock")]
        data: u8,
        inner: String,
    }
    #[test]
    fn test_token_name() {
        let mt = MockToken {
            data: 4,
            inner: String::from("abab"),
        };

        assert_eq!("mcok", mt.name());

        let mock = mt.get_data("mock");
        println!("{:?}", mock);
        assert!(mock.is_some());

        let mock = mt.get_data("inner");
        println!("{:?}", mock);
        assert!(mock.is_some());

        let all = mt.get_all_name();
        println!("{:?}", all);
        assert_eq!(vec!["mock", "inner"], all);

        let boxy: Box<dyn FullToken> = Box::new(mt);

        let from_box = MockToken::from_token(boxy);

        assert!(from_box.is_some());
        println!("{:?}", from_box);
    }
}
