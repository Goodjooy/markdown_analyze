use crate::dfa::interface::{FullToken, TokenTrait};

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

token_generator![
    "sep_char": SepChar,       // ` `
    "change_line": ChangeLine, // \n
    "new_param": NewParam,     // `  \n`
    "idented": Idented,        // `    ` | \t
    // 标题部分
    "h1": Title1, //#
    "h2": Title2, //##
    "h3": Title3, // ###
    "h4": Title4, // ####
    "h5": Title5, // #####
    "h6": Title6, // ######
    // orthers
    "refer": Reference,          // >
    "link_start": LinkStart,     // [
    "img_start": ImgStart,       // ![
    "box_mid": BoxMid,           // ](
    "box_end": BoxEnd,           // )
    "unorder_list": UnorderList, // -
    "order_list": OrderList,     // 0|1|2|3|4|5|6|7|8|9 .
    "SeperLine": SepLine,        // ---
    "star": Star,                // *
    "d_star": DoubleStar,        //**
    "t_star": TripleStar         // ***
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
