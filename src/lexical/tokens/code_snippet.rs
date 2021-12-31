use full_token_derive_macro::FullToken;

use super::super::token_trait::{FullToken, TokenTrait};

pub struct PartCodeSnippet;

impl TokenTrait for PartCodeSnippet {
    fn name(&self) -> &'static str {
        "Code Snip"
    }

    fn to_full(&self, buff: &[char]) -> Box<dyn FullToken> {
        // buff长度为2 表明是空白
        let inner = if buff.len() == 2 {
            String::new()
        }
        // buff 长度小于6 不是复杂代码块
        else if buff.len() < 6 {
            String::from_iter(buff[1..buff.len() - 1].iter())
        }
        // 以三重反引号"`"包围部分
        else if buff[0..3] == ['`', '`', '`']
            && buff[buff.len() - 3..buff.len()] == ['`', '`', '`']
        {
            String::from_iter(buff[3..buff.len() - 3].iter())
        }
        // 以一对反引号包围的部分
        else if buff[0] == '`' && buff[buff.len() - 1] == '`' {
            String::from_iter(buff[1..buff.len() - 1].iter())
        } else {
            unreachable!()
        };
        Box::new(CodeSnippet { inner })
    }
}
#[derive(FullToken)]
#[token(name = "code_snippet")]
pub struct CodeSnippet {
    pub inner: String,
}

#[cfg(test)]
mod test {
    use crate::lexical::token_trait::FromTokenMeta;

    use super::*;

    #[test]
    fn test_empty() {
        let buff = "``".chars().collect::<Vec<_>>();
        let res = PartCodeSnippet.to_full(&buff);

        assert_eq!(
            "",
            String::from_token_meta(&res.get_data("inner").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_not_empty() {
        let buff = "`ababb`".chars().collect::<Vec<_>>();
        let res = PartCodeSnippet.to_full(&buff);

        assert_eq!(
            "ababb",
            String::from_token_meta(&res.get_data("inner").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_empty_cmp() {
        let buff = "``````".chars().collect::<Vec<_>>();
        let res = PartCodeSnippet.to_full(&buff);

        assert_eq!(
            "",
            String::from_token_meta(&res.get_data("inner").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_not_empty_cmp() {
        let buff = "```abba```".chars().collect::<Vec<_>>();
        let res = PartCodeSnippet.to_full(&buff);

        assert_eq!(
            "abba",
            String::from_token_meta(&res.get_data("inner").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_not_empty_cmp_with_inner() {
        let buff = "```a`bba```".chars().collect::<Vec<_>>();
        let res = PartCodeSnippet.to_full(&buff);

        assert_eq!(
            "a`bba",
            String::from_token_meta(&res.get_data("inner").unwrap()).unwrap()
        );
    }
}
