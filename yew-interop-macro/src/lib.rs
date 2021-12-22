use convert_case::{Case, Casing};
use itertools::izip;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::borrow::Cow;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error as SynError, Ident, LitInt, LitStr, Result as SynResult};
use yew_interop_core::{Link, LinkType};

struct ResourceDeclaration {
    idents: Vec<Ident>,
    link_groups: Vec<Vec<Link>>,
}

impl Parse for ResourceDeclaration {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut idents = Vec::new();
        let mut link_groups = Vec::new();
        while input.peek(Ident) {
            let ident = input.parse::<Ident>().unwrap();
            idents.push(ident);

            let mut links = Vec::new();
            while input.peek(LitStr) {
                let lit_str: LitStr = input.parse::<LitStr>().unwrap();
                let src: String = lit_str.value();
                let r#type = if src.ends_with(".js") {
                    LinkType::Js
                } else if src.ends_with(".css") {
                    LinkType::Css
                } else {
                    return Err(SynError::new(
                        lit_str.span(),
                        "Url needs to end with .js or .css",
                    ));
                };
                links.push(Link {
                    r#type,
                    src: Cow::from(src),
                })
            }
            link_groups.push(links)
        }

        Ok(Self {
            idents,
            link_groups,
        })
    }
}

/// Declare your resources as whitespace separated groups of `resource identifier`
/// and one or more `resource url`.
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
    let ResourceDeclaration {
        idents,
        link_groups,
    } = parse_macro_input!(input as ResourceDeclaration);

    let (resource_names, resource_name_spans): (Vec<_>, Vec<_>) =
        idents.iter().map(|i| (i.to_string(), i.span())).unzip();

    let handle_idents = resource_names.iter().map(|name| {
        Ident::new(
            &format!("{}LinkGroupStatusHandle", name.to_case(Case::Pascal)),
            Span::call_site(),
        )
    });

    let hooks = izip!(
        resource_names.iter(),
        resource_name_spans,
        link_groups.iter(),
        handle_idents.clone()
    )
    .map(|(resource_name, span, links, handle_ident)| {
        let ident = format!("use_{}", resource_name);
        let ident = Ident::new(&ident, span);

        let links = links.iter().map(|link| {
            let r#type = match link.r#type {
                LinkType::Css => quote! {Css},
                LinkType::Js => quote! {Js},
            };

            let lit_str = LitStr::new(link.src.as_ref(), Span::call_site());
            quote! {
                yew_interop::Link {
                    r#type: ::yew_interop::LinkType::#r#type,
                    src: ::std::borrow::Cow::from(#lit_str),
                }
            }
        });
        let handle_ident_one = handle_ident.clone();
        let handle_ident_two = handle_ident.clone();
        let handle_ident_three = handle_ident.clone();

        quote! {
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

    let remaining_idents = resource_names.iter().map(|name| {
        Ident::new(
            &format!("{}_REMAINING", name.to_case(Case::UpperSnake)),
            Span::call_site(),
        )
    });
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

    let expanded = quote! {

        #(
            #hooks
        )*

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
                                    ::yew_interop::LinkType::Js => ::yew::html! {
                                        <script {src} type="text/javascript" onload={onload.clone()}/>
                                    },
                                    ::yew_interop::LinkType::Css => ::yew::html! {
                                        <link rel="stylesheet" type="text/css" href={src} onload={onload.clone()}/>
                                    }
                                }
                                }
                            )
                        }
                    </>
                }
            };


            let produce_not_completed_links = |links: Vec<::yew_interop::Link>, dispatcher: ::yew::UseReducerDispatcher<::yew_interop::LinkGroupStatus>| {
                let onload = move |_| {
                    TOAST_REMAINING.with(|r|{
                        let current = *r.borrow();

                        *r.borrow_mut() = current - 1;
                        if current > 1 {
                            *r.borrow_mut() = current - 1
                        } else {
                            dispatcher.dispatch(::yew_interop::LinkGroupStatusAction::Completed)
                        }
                    })
                };

                make_links_with_onload(links, Some(::yew::Callback::from(onload)))
            };

            #(
                let (#handle_tmp_idents, #link_element_tmp_idents) = match &*#reducer_idents_one {
                    ::yew_interop::LinkGroupStatus::Started {links} => (#handle_idents_one::Started,
                        {
                let dispatcher = #reducer_idents_two.dispatcher();
                let onload = move |_| {
                    #remaining_idents_one.with(|r|{
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
            }
                                                         ),
                    ::yew_interop::LinkGroupStatus::Completed{links} => {
                        (#handle_idents_two::Completed,
                                                          make_links_with_onload(links.clone(), None))},
                    ::yew_interop::LinkGroupStatus::NotRequested => (#handle_idents_three::NotRequested(#reducer_idents_three.dispatcher()), ::yew::html!{})
                };
            )*
            ::yew::html! {
                <>
                    #(
                        <::yew::ContextProvider<#handle_idents_four> context={#handle_tmp_idents_one}>
                    )*
                        {for props.children.iter()}
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


    };

    expanded.into()
}
