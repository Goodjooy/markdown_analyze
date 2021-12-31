
#[macro_export]
macro_rules! token_generator {
    ($name:literal,$ty:ident) => {
        #[derive(Clone)]
        pub struct $ty;

        impl FullToken for $ty{
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
pub mod titles;
pub mod reference;

use crate::lexical::token_trait::TokenTrait;
use crate::lexical::FullToken;
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
    "star": Star               // *
];

pub struct FullTrans(char);
pub struct Trans;
impl FullToken for FullTrans {}

impl TokenTrait for Trans {
    fn name(&self) -> &'static str {
        "Trans"
    }

    fn to_full(&self, buff: &[char]) -> Box<dyn FullToken> {
        Box::new(FullTrans(buff[1]))
    }
}

pub struct Plain(pub(super) String);
impl FullToken for Plain {
    
}

pub struct  Eof;
impl FullToken for Eof {
    
} 