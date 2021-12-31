#[macro_export]
macro_rules! token_generator {
    ($name:literal,$ty:ident) => {
        #[derive(Clone)]
        pub struct $ty;

        impl FromToken for $ty {
            fn token_name() -> &'static str { $name }
            fn from_token(src: std::boxed::Box<(dyn FullToken)>) -> Option<Self> {
                if src.name()==Self::token_name(){
                    Some($ty)
                }else{
                    None
                }
            }
        }

        impl FullToken for $ty{
            fn name(&self) -> &'static str { $name }
        }

        impl TokenTrait for $ty{
            fn name(&self) -> &'static str {
                $name
            }
            fn to_full(&self,_:&[char])->Box<dyn FullToken>{
                Box::new(self.clone())
            }
        }
    };

    [ $($name:literal : $ty:ident),* ]=>{
        $(
            token_generator!($name,$ty);
        )*
    }
}

pub mod code_snippet;
pub mod reference;
pub mod titles;

use crate::lexical::token_trait::FromToken;
use crate::lexical::token_trait::TokenTrait;
use crate::lexical::FullToken;
use full_token_derive_macro::FullToken;
pub use reference::PartRef as Reference;

token_generator![
    "sep_char": SepChar,       // ` `
    "change_line": ChangeLine, // \n
    "new_param": NewParam,     // `  \n`
    "idented": Idented,        // `    ` | \t
    // orthers
    "link_start": LinkStart,     // [
    "img_start": ImgStart,       // ![
    "box_mid": BoxMid,           // ](
    "box_end": BoxEnd,           // )
    "unorder_list": UnorderList, // -
    "order_list": OrderList,     // 0|1|2|3|4|5|6|7|8|9 .
    "SeperLine": SepLine,        // ---
    "star": Star,                // *
    // 特殊token
    "eof": Eof
];

#[derive(FullToken)]
pub struct FullTrans {
    ch: char,
}
pub struct Trans;

impl TokenTrait for Trans {
    fn name(&self) -> &'static str {
        "Trans"
    }

    fn to_full(&self, buff: &[char]) -> Box<dyn FullToken> {
        Box::new(FullTrans { ch: buff[1] })
    }
}

#[derive(FullToken)]
pub struct Plain {
    pub(super) inner: String,
}

impl Plain {
    pub fn new(buff: &[char]) -> Self {
        Self {
            inner: buff.iter().collect(),
        }
    }
    pub fn new_box(buff: &[char]) -> Box<dyn FullToken> {
        Box::new(Self {
            inner: buff.iter().collect(),
        })
    }
}
