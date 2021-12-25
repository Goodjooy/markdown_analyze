use proc_macro::TokenStream;
use syn::{ext::IdentExt, parse::Parse, parse_macro_input, token::Match, ExprLit, Ident, Token};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct RegexToken {
    name: Ident,
    sign: Token!(=>),
    expr: syn::ExprLit,
}

impl Parse for RegexToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let sign: Token!(=>) = input.parse()?;
        let expr: ExprLit = input.parse()?;

        Ok(Self { name, sign, expr })
    }
}

#[proc_macro]
pub fn regex_prase(input: TokenStream) -> TokenStream {
    let a: Match;
    let data = parse_macro_input!(input as RegexToken);
    unimplemented!()
}
