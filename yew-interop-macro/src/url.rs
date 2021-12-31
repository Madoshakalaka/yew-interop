use itertools::{Either, Itertools};
use proc_macro::Ident;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Error as SynError, Expr, ExprLit, Lit, LitStr, Result as SynResult};
use yew_interop_core::LinkType;

pub struct LibraryUrl {
    pub(crate) url: Url,
    pub(crate) link_type: LinkType,
}

impl LibraryUrl {
    pub(crate) fn new(url: Url, link_type: LinkType) -> LibraryUrl {
        Self { url, link_type }
    }
}

impl TryFrom<LitStr> for LibraryUrl {
    type Error = SynError;

    /// returns [`syn::Error`] when the url doesn't end with .js or .css
    fn try_from(lit_str: LitStr) -> Result<Self, Self::Error> {
        let src: String = lit_str.value();

        if let Some(link_type) = src
            .ends_with(".js")
            .then(|| LinkType::Js)
            .or_else(|| src.ends_with(".css").then(|| LinkType::Css))
        {
            Ok(Self {
                url: Url::UnSpecified(lit_str),
                link_type,
            })
        } else {
            Err(SynError::new(
                lit_str.span(),
                "url needs to end with .js or .css,\
                               otherwise you should specify js or css explicitly before the url",
            ))
        }
    }
}

pub(crate) enum Url {
    UnSpecified(LitStr),
    TypeSpecified(Expr),
}

impl ToTokens for Url {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Url::UnSpecified(l) => l.to_tokens(tokens),
            Url::TypeSpecified(e) => e.to_tokens(tokens),
        }
    }
}
