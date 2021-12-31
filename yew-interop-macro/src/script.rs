use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Expr, Ident, Token};


pub(crate) struct EffectScriptEntry {
    pub ident: Ident,
    pub url: Expr,
}

impl Parse for EffectScriptEntry {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.parse::<Token![!]>().unwrap();
        let ident = input.parse::<Ident>().unwrap();
        let url = input.parse::<Expr>()?;
        Ok(Self { ident, url })
    }
}
