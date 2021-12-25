use crate::dfa::TokenTrait;

#[macro_export]
macro_rules! token_generator {
    ($name:literal,$ty:ident) => {
        pub struct $ty;

        impl TokenTrait for $ty{
            fn name(&self) -> &'static str {
                $name
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
    "order_list": OrderList,    // 0|1|2|3|4|5|6|7|8|9 .
    "SeperLine": SepLine,        // ---
    "star": Star,                // *
    "d_star": DoubleStar,        //**
    "t_star": TribleStar         // ***
];
