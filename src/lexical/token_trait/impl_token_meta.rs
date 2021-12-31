use super::{FromTokenMeta, IntoTokenMeta, Number, TokenMeta};

impl IntoTokenMeta for u8 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::P(self as u64))
    }
}
impl IntoTokenMeta for u16 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::P(self as u64))
    }
}
impl IntoTokenMeta for u32 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::P(self as u64))
    }
}
impl IntoTokenMeta for u64 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::P(self))
    }
}

impl FromTokenMeta for u8 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::P(u)) = src {
            Some(*u as u8)
        } else {
            None
        }
    }
}

impl FromTokenMeta for u16 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::P(u)) = src {
            Some(*u as u16)
        } else {
            None
        }
    }
}

impl FromTokenMeta for u32 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::P(u)) = src {
            Some(*u as u32)
        } else {
            None
        }
    }
}
impl FromTokenMeta for u64 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::P(u)) = src {
            Some(*u)
        } else {
            None
        }
    }
}

impl IntoTokenMeta for i8 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::N(self as i64))
    }
}
impl IntoTokenMeta for i16 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::N(self as i64))
    }
}
impl IntoTokenMeta for i32 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::N(self as i64))
    }
}
impl IntoTokenMeta for i64 {
    fn into_token_meta(self) -> super::TokenMeta {
        TokenMeta::Num(Number::N(self))
    }
}

impl FromTokenMeta for i8 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::N(u)) = src {
            Some(*u as i8)
        } else {
            None
        }
    }
}

impl FromTokenMeta for i16 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::N(u)) = src {
            Some(*u as i16)
        } else {
            None
        }
    }
}

impl FromTokenMeta for i32 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::N(u)) = src {
            Some(*u as i32)
        } else {
            None
        }
    }
}
impl FromTokenMeta for i64 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::N(u)) = src {
            Some(*u)
        } else {
            None
        }
    }
}

impl IntoTokenMeta for f32 {
    fn into_token_meta(self) -> TokenMeta {
        TokenMeta::Num(Number::FShort(self))
    }
}

impl IntoTokenMeta for f64 {
    fn into_token_meta(self) -> TokenMeta {
        TokenMeta::Num(Number::F(self))
    }
}

impl FromTokenMeta for f32 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::FShort(f)) = src {
            Some(*f)
        } else {
            None
        }
    }
}
impl FromTokenMeta for f64 {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Num(Number::F(f)) = src {
            Some(*f)
        } else {
            None
        }
    }
}

impl IntoTokenMeta for bool {
    fn into_token_meta(self) -> TokenMeta {
        TokenMeta::Bool(self)
    }
}

impl FromTokenMeta for bool {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Bool(b) = src {
            Some(*b)
        } else {
            None
        }
    }
}

impl IntoTokenMeta for char {
    fn into_token_meta(self) -> TokenMeta {
        TokenMeta::Char(self)
    }
}

impl FromTokenMeta for char {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Char(b) = src {
            Some(*b)
        } else {
            None
        }
    }
}

impl IntoTokenMeta for String {
    fn into_token_meta(self) -> TokenMeta {
        TokenMeta::Str(self)
    }
}

impl FromTokenMeta for String {
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Str(b) = src {
            Some(b.clone())
        } else {
            None
        }
    }
}

impl<T> IntoTokenMeta for Vec<T>
where
    T: IntoTokenMeta,
{
    fn into_token_meta(self) -> TokenMeta {
        TokenMeta::Vec(
            self.into_iter()
                .map(IntoTokenMeta::into_token_meta)
                .collect(),
        )
    }
}

impl<T> FromTokenMeta for Vec<T>
where
    T: FromTokenMeta,
{
    fn from_token_meta(src: &TokenMeta) -> Option<Self> {
        if let TokenMeta::Vec(v) = src {
            Some(v.iter().map(T::from_token_meta).filter_map(|d| d).collect())
        } else {
            None
        }
    }
}
