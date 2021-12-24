
/// # markdown load unit md文件加载语法单元
pub trait MDMeta:serde::Serialize {
    /// the Md meta name, ***MUST*** be a `static str`
    /// Md 语法单元名称，必须是static 字符串
    fn name(&self)->&'static str;
}

type finalstruct=Vec<Box<dyn MDMeta>>;


pub enum LoadResult<T> {
    Loaded(T),

}
/// md文件的句法单元
pub trait MdTokenMeta:serde::Serialize {
}
#[derive(serde::Serialize)]
struct Title{
    level:TitleToken,
    title:String,
}

#[derive(serde::Serialize)]
struct TitleToken {
    level:u8
}

impl MdTokenMeta for TitleToken {
    
}

