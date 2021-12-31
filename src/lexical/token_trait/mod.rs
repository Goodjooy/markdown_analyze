use serde::ser::SerializeStruct;

mod impl_token_meta;

pub trait TokenTrait {
    fn name(&self) -> &'static str;
    fn to_full(&self, buff: &[char]) -> Box<dyn FullToken>;
}

pub trait FullToken {
    fn name(&self) -> &'static str {
        ""
    }
    fn get_data(&self, _name: &str) -> Option<&TokenMeta> {
        None
    }
    fn get_all_name(&self) -> Vec<&'static str> {
        vec![]
    }
    fn all_data(&self) -> Vec<(&'static str, &TokenMeta)> {
        vec![]
    }
}

pub trait FromToken:Sized {
    fn from_token(src:Box<dyn FullToken>)->Option<Self>;
}


#[derive(Clone, Copy)]
pub enum Number {
    P(u64),
    N(i64),
    FShort(f32),
    F(f64),
}

#[derive(Clone)]
pub enum TokenMeta {
    Num(Number),
    Bool(bool),
    Char(char),
    Str(String),
    Vec(Vec<TokenMeta>),
}

pub trait IntoTokenMeta {
    fn into_token_meta(self) -> TokenMeta;
}

pub trait FromTokenMeta:Sized {
    fn from_token_meta(src: &TokenMeta) -> Option<Self>;
}

impl serde::Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Number::P(n) => {
                if let Ok(u) = TryInto::<u8>::try_into(*n) {
                    serializer.serialize_u8(u)
                } else if let Ok(u) = TryInto::<u16>::try_into(*n) {
                    serializer.serialize_u16(u)
                } else if let Ok(v) = TryInto::<u32>::try_into(*n) {
                    serializer.serialize_u32(v)
                } else {
                    serializer.serialize_u64(*n)
                }
            }
            Number::N(i) => {
                if let Ok(u) = TryInto::<i8>::try_into(*i) {
                    serializer.serialize_i8(u)
                } else if let Ok(u) = TryInto::<i16>::try_into(*i) {
                    serializer.serialize_i16(u)
                } else if let Ok(v) = TryInto::<i32>::try_into(*i) {
                    serializer.serialize_i32(v)
                } else {
                    serializer.serialize_i64(*i)
                }
            }
            Number::FShort(f) => serializer.serialize_f32(*f),
            Number::F(f) => serializer.serialize_f64(*f),
        }
    }
}

impl serde::Serialize for TokenMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TokenMeta::Num(n) => n.serialize(serializer),
            TokenMeta::Bool(b) => b.serialize(serializer),
            TokenMeta::Char(c) => c.serialize(serializer),
            TokenMeta::Str(s) => s.serialize(serializer),
            TokenMeta::Vec(v) => v.serialize(serializer),
        }
    }
}

impl serde::Serialize for dyn FullToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let datas = self.all_data();
        let mut s = serializer.serialize_struct(self.name(), datas.len())?;
        for (k, v) in datas {
            s.serialize_field(k, v)?;
        }
        s.end()
    }
}
