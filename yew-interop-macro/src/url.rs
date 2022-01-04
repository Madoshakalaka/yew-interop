use proc_macro2::TokenStream;
use quote::ToTokens;

use syn::{Error as SynError, Expr, LitStr};
use url::{ParseError, Url};
use yew_interop_core::LinkType;

pub struct LibraryUrl {
    pub(crate) url: UrlInput,
    pub(crate) link_type: LinkType,
}

impl LibraryUrl {
    pub(crate) fn new(url: UrlInput, link_type: LinkType) -> LibraryUrl {
        Self { url, link_type }
    }
}

impl TryFrom<LitStr> for LibraryUrl {
    type Error = SynError;

    /// returns [`syn::Error`] when the url doesn't end with .js or .css
    fn try_from(lit_str: LitStr) -> Result<Self, Self::Error> {
        let src: String = lit_str.value();
        let url = Url::parse(&src).unwrap_or_else(|e| {
            match e {
                ParseError::RelativeUrlWithoutBase => {
                    let pseudo_url =
                        Url::parse("https://madoshakalaka.github.io/yew-interop/master").unwrap();
                    Ok(pseudo_url.join(&src).unwrap())
                }
                e => Err(e),
            }
            .unwrap()
        });
        let last_path_segment = url.path_segments().unwrap().last().unwrap();

        if let Some(link_type) = last_path_segment
            .ends_with(".js")
            .then(|| LinkType::Js)
            .or_else(|| last_path_segment.ends_with(".css").then(|| LinkType::Css))
        {
            Ok(Self {
                url: UrlInput::UnSpecified(lit_str),
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

pub(crate) enum UrlInput {
    UnSpecified(LitStr),
    TypeSpecified(Box<Expr>),
}

impl ToTokens for UrlInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            UrlInput::UnSpecified(l) => l.to_tokens(tokens),
            UrlInput::TypeSpecified(e) => e.to_tokens(tokens),
        }
    }
}
