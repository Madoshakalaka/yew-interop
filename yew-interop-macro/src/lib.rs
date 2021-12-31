mod kw;
#[cfg(feature = "script")]
mod script;
mod url;

#[cfg(feature = "script")]
use script::EffectScriptEntry;

use crate::url::{LibraryUrl, Url};
use itertools::{izip, Either};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use syn::parse::{Parse, ParseStream};

use syn::{
    parse_macro_input, Error as SynError, Expr, ExprLit, Ident, Lit, LitInt, LitStr,
    Result as SynResult, Token,
};
use yew_interop_core::LinkType;

struct ResourceDeclaration {
    idents: Vec<Ident>,
    link_groups: Vec<Vec<LibraryUrl>>,
    #[cfg(feature = "script")]
    effect_scripts: Vec<EffectScriptEntry>,
}

#[cfg(feature = "script")]
enum NextEntry {
    EffectScript,
    Lib,
}

#[cfg(feature = "script")]
fn peek_script_or_lib(input: ParseStream) -> SynResult<NextEntry> {
    let lookahead = input.lookahead1();

    lookahead
        .peek(Token![!])
        .then(|| NextEntry::EffectScript)
        .or_else(|| lookahead.peek(Ident).then(|| NextEntry::Lib))
        .ok_or_else(|| lookahead.error())
}

fn parse_library_urls(input: ParseStream) -> SynResult<Vec<LibraryUrl>> {
    let mut urls = Vec::new();
    loop {
        if input.peek(kw::js) {
            input.parse::<kw::js>().unwrap();
            let expr = input.parse::<Expr>()?;
            urls.push(LibraryUrl::new(
                Url::TypeSpecified(Box::new(expr)),
                LinkType::Js,
            ))
        } else if input.peek(kw::css) {
            input.parse::<kw::css>().unwrap();
            let expr = input.parse::<Expr>()?;
            urls.push(LibraryUrl::new(
                Url::TypeSpecified(Box::new(expr)),
                LinkType::Css,
            ))
        } else if input.peek(LitStr) {
            urls.push(LibraryUrl::try_from(input.parse::<LitStr>().unwrap())?);
        } else {
            break;
        }
    }
    Ok(urls)
}

impl Parse for ResourceDeclaration {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut idents = Vec::new();
        let mut link_groups = Vec::new();

        #[cfg(feature = "script")]
        let mut effect_scripts = Vec::new();

        while !input.is_empty() {
            #[cfg(feature = "script")]
            match peek_script_or_lib(input)? {
                NextEntry::EffectScript => {
                    let entry = EffectScriptEntry::parse(input)?;
                    effect_scripts.push(entry)
                }
                NextEntry::Lib => {
                    Self::parse_library(input, &mut idents, &mut link_groups)?;
                }
            }
            #[cfg(not(feature = "script"))]
            Self::parse_library(input, &mut idents, &mut link_groups)?;
        }

        Ok(Self {
            idents,
            link_groups,

            #[cfg(feature = "script")]
            effect_scripts,
        })
    }
}

impl ResourceDeclaration {
    fn parse_library(
        input: ParseStream,
        idents: &mut Vec<Ident>,
        link_groups: &mut Vec<Vec<LibraryUrl>>,
    ) -> SynResult<()> {
        let ident = input.parse::<Ident>().unwrap();
        idents.push(ident);
        link_groups.push(parse_library_urls(input)?);
        Ok(())
    }
}

/// Declare your libraries as whitespace separated groups of identifier
/// and one or more urls
///
/// # Example 1
///
/// ```no_run
/// declare_resources!(my_library "https://cdn.com/my_library.js")
/// ```
///
/// # Example 2
///
/// ```no_run
/// declare_resources!(
/// library_one
/// "https://cdn.com/a.js"
/// "https://cdn.com/b.css"
/// "https://cdn.com/c.css"
/// library_two
/// "https://cdn.com/b.css"
/// )
/// ```
///
/// # Explicitly Specify the Url Type
///
/// the macro needs to know whether the url is JavaScript or CSS.
/// When you provide a string literal as the examples above,
/// the macro derives the information from the suffix (either .js or .css).
/// When the string literal doesn't end with .js or .css,
/// or when you provide other expressions like a macro call or a identifier,
/// you need to manually specify the URL type by prepending the custom keyword js/css
/// before the url.
///
/// ```no_run
/// declare_resources!(
/// my_library
/// css MY_CSS_URL
/// js static_url!("my_library")
/// js get_url("my_other_library")
/// )
/// ```
///
/// The macro expect the return type of the expression to implement `Into<Cow<'static, str>>`,
/// `&'static str`, `String` and `Cow<'static, str>` are all valid types for example.
///
/// # Side Effect Scripts
///
/// To declare a side effect script, just prepend the identifier with an exclamation mark (!),
/// note the script has to be in JavaScript, so no type should be explicitly specified.
///
///
/// ```no_run
/// declare_resources!(
/// my_library
/// "https://cdn.com/lib.js"
/// ! my_effect
/// "https://cdn.com/effect.js"
/// )
/// ```
///
/// # Consumption
///
/// The macro expands into a `<ResourceProvider/>` component and hook functions for each of your
/// resources.
/// The names of the hook functions are `use_<resource_identifier>`.
/// Example 2 above will expand into two hook functions `pub fn use_library_one()`
/// and `pub fn use_library_two()`
///
/// You should wrap the root component of your app in the `<ResourceProvider/>` like this:
/// ```no_run
/// html!{
///     <ResourceProvider>
///         <App/>
///     </ResourceProvider>
/// }
/// ```
///
/// The hooks are to be used in the consuming components.
#[proc_macro]
pub fn declare_resources(input: TokenStream) -> TokenStream {
    let resource_declaration = parse_macro_input!(input as ResourceDeclaration);

    #[cfg(not(feature = "script"))]
    let ResourceDeclaration {
        idents,
        link_groups,
    } = resource_declaration;

    #[cfg(feature = "script")]
    let ResourceDeclaration {
        idents,
        link_groups,
        effect_scripts,
    } = resource_declaration;

    #[cfg(feature = "script")]
    let (script_hooks, script_handle_enums, script_urls, script_loaders, script_handles): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = itertools::multiunzip(effect_scripts.into_iter().map(
        |EffectScriptEntry { ident, url }| {
            let ident_string = ident.to_string();

            (
                Ident::new(&format!("use_{}", ident_string), ident.span()),
                Ident::new(&format!("{}ScriptHandle", ident_string), Span::call_site()),
                url,
                Ident::new(
                    &format!("{}_script_loader", ident_string),
                    Span::call_site(),
                ),
                Ident::new(
                    &format!("{}_script_handle", ident_string),
                    Span::call_site(),
                ),
            )
        },
    ));

    let (resource_names, resource_name_spans): (Vec<_>, Vec<_>) =
        idents.iter().map(|i| (i.to_string(), i.span())).unzip();

    let handle_idents = resource_names
        .iter()
        .map(|name| Ident::new(&format!("{}LinkGroupStatusHandle", name), Span::call_site()));

    let library_hooks = izip!(
        resource_names.iter(),
        resource_name_spans,
        link_groups.iter(),
        handle_idents.clone()
    )
    .map(|(resource_name, span, links, handle_ident)| {
        let ident = format!("use_{}", resource_name);
        let ident = Ident::new(&ident, span);

        let links = links.iter().map(|LibraryUrl { link_type, url }| {
            let r#type = match link_type {
                LinkType::Css => quote! {Css},
                LinkType::Js => quote! {Js},
            };

            quote! {
                yew_interop::Link {
                    r#type: ::yew_interop::LinkType::#r#type,
                    src: ::std::borrow::Cow::from(#url),
                }
            }
        });
        let handle_ident_one = handle_ident.clone();
        let handle_ident_two = handle_ident.clone();
        let handle_ident_three = handle_ident.clone();

        quote! {

            /// Request the library to be loaded.
            /// Returns None when the library is first requested or
            /// is requested elsewhere but not loaded yet.
            /// The component will get notified when the library is ready.
            pub fn #ident() -> bool{
                let handle = ::yew::use_context::<#handle_ident_one>().unwrap();
                match handle {
                    #handle_ident_two::NotRequested(disp) => {
                        disp.dispatch(::yew_interop::LinkGroupStatusAction::PleaseStart(vec![
                            #(
                                #links,
                            )*
                        ]));
                        false
                    }
                    #handle_ident_three::Started => false,
                    #handle_ident::Completed => true
                }
            }
        }
    });

    #[cfg(feature = "script")]
    let script_hooks = {
        let script_handle_enums_one = script_handle_enums.iter();
        let script_handle_enums_two = script_handle_enums_one.clone();
        let script_handle_enums_three = script_handle_enums_one.clone();
        let script_handle_enums_four = script_handle_enums_one.clone();
        quote! {

            #(
                /// Request the script to be loaded.
                /// Returns None when the script is first requested or is requested elsewhere but not loaded yet.
                /// The component will get notified when the script is ready.
                pub fn #script_hooks() -> Option<::yew_interop::script::Script> {
                    let handle = ::yew::use_context::<#script_handle_enums_four>().unwrap();
                    match handle {
                        #script_handle_enums_one::NotRequested(disp) => {
                            disp.dispatch(::yew_interop::script::ScriptLoaderAction::Start);
                            ::yew_interop::script::wasm_bindgen_futures::spawn_local(async move {
                                let script = ::yew_interop::script::fetch_script(#script_urls.into()).await;
                                disp.dispatch(::yew_interop::script::ScriptLoaderAction::Finish(
                                    ::std::rc::Rc::new(script),
                                ));
                            });
                            None
                        }
                        #script_handle_enums_two::Started => None,
                        #script_handle_enums_three::Completed(s) => Some(s),
                    }
                }
            )*


        }
    };
    #[cfg(not(feature = "script"))]
    let script_hooks = quote! {};

    #[cfg(feature = "script")]
    let script_handle_enums_ts = {
        let script_handle_enums = script_handle_enums.iter();
        quote! {

            #(
                #[derive(Clone, PartialEq)]
                enum #script_handle_enums {
                    NotRequested(::yew::UseReducerDispatcher<::yew_interop::script::ScriptLoader>),
                    Started,
                    Completed(::yew_interop::script::Script),
                }
            )*

        }
    };
    #[cfg(not(feature = "script"))]
    let script_handle_enums_ts = quote! {};

    #[cfg(feature = "script")]
    let script_loaders_and_handles = {
        let script_loaders_one = script_loaders.iter();
        let script_loaders_two = script_loaders_one.clone();
        let script_handle_enums = script_handle_enums.iter();
        let script_handle_enums_one = script_handle_enums.clone();
        let script_handle_enums_two = script_handle_enums.clone();
        let script_handles = script_handles.iter();
        quote! {

            #(

                let #script_loaders =
                    ::yew::use_reducer(|| ::yew_interop::script::ScriptLoader::NotRequested);
                let #script_handles = match &*#script_loaders_one {
                    yew_interop::script::ScriptLoader::NotRequested => {
                        #script_handle_enums_two::NotRequested(#script_loaders_two.dispatcher())
                    }
                    yew_interop::script::ScriptLoader::Started => #script_handle_enums::Started,
                    yew_interop::script::ScriptLoader::Completed(s) => {
                        #script_handle_enums_one::Completed(s.clone())
                    }
                };

            )*


        }
    };
    #[cfg(not(feature = "script"))]
    let script_loaders_and_handles = quote! {};

    let handle_idents_one = handle_idents.clone();
    let handle_idents_two = handle_idents.clone();
    let handle_idents_three = handle_idents.clone();
    let handle_idents_four = handle_idents.clone();
    let handle_idents_five = handle_idents.clone().rev();

    let handle_enums = handle_idents.clone().map(|handle_ident| {
        quote! {
            #[derive(Clone, PartialEq)]
            enum #handle_ident {
                NotRequested(::yew::UseReducerDispatcher<::yew_interop::LinkGroupStatus>),
                Started,
                Completed,
            }
        }
    });

    let remaining_idents = resource_names
        .iter()
        .map(|name| Ident::new(&format!("{}_REMAINING", name), Span::call_site()));
    let remaining_idents_one = remaining_idents.clone();
    let remaining_count = link_groups
        .iter()
        .map(|link_group| LitInt::new(&link_group.len().to_string(), Span::call_site()));

    let reducer_idents = resource_names
        .iter()
        .map(|name| Ident::new(&format!("{}_link_group_status", name), Span::call_site()));

    let handle_tmp_idents = resource_names
        .iter()
        .map(|name| Ident::new(&format!("{}_link_handle", name), Span::call_site()));

    let handle_tmp_idents_one = handle_tmp_idents.clone();

    let link_element_tmp_idents = resource_names
        .iter()
        .map(|name| Ident::new(&format!("{}_links", name), Span::call_site()));

    let link_element_tmp_idents_one = link_element_tmp_idents.clone();
    let reducer_idents_one = reducer_idents.clone();
    let reducer_idents_two = reducer_idents.clone();
    let reducer_idents_three = reducer_idents.clone();

    let expanded = {
        #[cfg(feature = "script")]
        let script_context_opening_tags = {
            let script_handle_enums = script_handle_enums.iter();

            quote! {
                #(
                    <::yew::ContextProvider<#script_handle_enums> context={#script_handles}>
                )*
            }
        };
        #[cfg(not(feature = "script"))]
        let script_context_opening_tags = quote! {};

        #[cfg(feature = "script")]
        let script_context_closing_tags = {
            let script_handle_enums = script_handle_enums.into_iter().rev();
            quote! {
                    #(
                        </::yew::ContextProvider<#script_handle_enums>>
                    )*
            }
        };
        #[cfg(not(feature = "script"))]
        let script_context_closing_tags = quote! {};

        quote! {

            #(
                #library_hooks
            )*

            #script_hooks

            #script_handle_enums_ts

            #(
                #handle_enums
            )*

            #[derive(::yew::Properties, PartialEq)]
            pub struct ResourceProviderProps {
                pub children: ::yew::Children,
            }

            #[::yew::function_component(ResourceProvider)]
            pub fn resource_provider(props: &ResourceProviderProps) -> Html {
                #(
                    let #reducer_idents = ::yew::use_reducer(::yew_interop::LinkGroupStatus::default);
                )*

                thread_local!{
                    #(
                        static #remaining_idents: ::std::cell::RefCell<u8> = ::std::cell::RefCell::new(#remaining_count);
                    )*
                }

                let make_links_with_onload = |links: Vec<::yew_interop::Link>, onload: Option<::yew::Callback<::yew::Event>>|{
                    ::yew::html!{
                        <>
                            {for
                                links.into_iter().map(|link| {

                                    let src: ::yew::virtual_dom::AttrValue = link.src.clone().into();
                                    match link.r#type {
                                        ::yew_interop::LinkType::Js => ::yew::html_nested! {
                                            <script {src} type="text/javascript" onload={onload.clone()}/>
                                        },
                                        ::yew_interop::LinkType::Css => ::yew::html_nested! {
                                            <link rel="stylesheet" type="text/css" href={src} onload={onload.clone()}/>
                                        }
                                    }
                                    }
                                )
                            }
                        </>
                    }
                };

                let get_links =
                    |links: &Vec<::yew_interop::Link>,
                     link_group_status: &::yew::UseReducerHandle<::yew_interop::LinkGroupStatus>,
                     count: &'static ::std::thread::LocalKey<::std::cell::RefCell<u8>>| {
                        let dispatcher = link_group_status.dispatcher();
                        let onload = move |_| {
                            count.with(|r| {
                                let current = *r.borrow();
                                *r.borrow_mut() = current - 1;
                                if current > 1 {
                                    *r.borrow_mut() = current - 1
                                } else {
                                    dispatcher.dispatch(::yew_interop::LinkGroupStatusAction::Completed)
                                }
                            })
                        };
                        make_links_with_onload(links.clone(), Some(::yew::Callback::from(onload)))
                    };


                #(
                    let (#handle_tmp_idents, #link_element_tmp_idents) = match &*#reducer_idents_one {
                        ::yew_interop::LinkGroupStatus::Started {links} => (
                            #handle_idents_one::Started,
                            get_links(links, &#reducer_idents_two, &#remaining_idents_one)
                        ),
                        ::yew_interop::LinkGroupStatus::Completed{links} => {
                            (#handle_idents_two::Completed,
                                                              make_links_with_onload(links.clone(), None))},
                        ::yew_interop::LinkGroupStatus::NotRequested => (#handle_idents_three::NotRequested(#reducer_idents_three.dispatcher()), ::yew::html!{})
                    };
                )*

                #script_loaders_and_handles

                ::yew::html! {
                    <>
                        #(
                            <::yew::ContextProvider<#handle_idents_four> context={#handle_tmp_idents_one}>
                        )*

                        #script_context_opening_tags
                            {for props.children.iter()}
                        #script_context_closing_tags

                        #(
                            </::yew::ContextProvider<#handle_idents_five>>
                        )*
                        // temptation: use portal and render this into <head/>
                        //  it doesn't work because yew attaches listeners to body.
                        //  The onload listeners would never fire.
                        //  it's a current limitation of portals
                        #(
                            {#link_element_tmp_idents_one}
                        )*
                    </>
                }
            }


        }
    };

    expanded.into()
}

trait ExprExt {
    fn into_lit_str(self) -> SynResult<Either<LitStr, Expr>>;
}

impl ExprExt for syn::Expr {
    fn into_lit_str(self) -> SynResult<Either<LitStr, Self>> {
        match self {
            syn::Expr::Lit(ExprLit { lit, .. }) => match lit {
                Lit::Str(lit_str) => Ok(Either::Left(lit_str)),
                _ => Err(SynError::new(lit.span(), "expecting a string literal")),
            },
            other => Ok(Either::Right(other)),
        }
    }
}
