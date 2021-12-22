use std::rc::Rc;

pub use gloo_utils;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlHeadElement, HtmlScriptElement};
use yew::prelude::*;
pub use yew_interop_core::{Link, LinkType};
pub use yew_interop_macro::declare_resources;

/// Run javascript after a component is rendered.
/// Note the script will run for every render.
/// If you want to conditionally run the script, see [`use_conditional_script_effect`]
pub fn use_script_effect(src: &'static str) {
    use_effect(move || {
        let document = gloo_utils::document();
        let old = document
            .query_selector(&format!("script[src='{}']", src))
            .unwrap();
        if let Some(o) = old {
            o.remove();
        }

        insert_script(src, &document);

        || {}
    })
}

fn insert_script(src: &str, document: &Document) {
    if let Ok(script) = document.create_element("script") {
        let head: HtmlHeadElement = document.head().unwrap();
        let script: HtmlScriptElement = script.unchecked_into();
        script.set_type("text/javascript");
        script.set_src(src);
        head.append_child(&script).unwrap();
    }
}

/// Run a script conditionally.
///
/// `should_run` receives `d` as the argument and returns a `bool`.
/// The script will only run when `should_rerun` returns true.
pub fn use_conditional_script_effect<Dep, Filter>(src: &'static str, should_run: Filter, d: Dep)
where
    Dep: PartialEq + 'static,
    Filter: FnOnce(&Dep) -> bool + 'static,
{
    use_effect_with_deps(
        move |d| {
            let document = gloo_utils::document();
            let old = document
                .query_selector(&format!("script[src='{}']", src))
                .unwrap();
            let should_reload = should_run(d);
            if let Some(o) = old {
                if should_reload {
                    o.remove();
                }
            }

            if should_reload {
                insert_script(src, &document)
            }

            || {}
        },
        d,
    );
}

pub enum LinkGroupStatusAction {
    PleaseStart(Vec<Link>),
    Completed,
}

#[derive(PartialEq, Clone)]
pub enum LinkGroupStatus {
    NotRequested,
    Started { links: Vec<Link> },
    Completed { links: Vec<Link> },
}

impl Default for LinkGroupStatus {
    fn default() -> Self {
        Self::NotRequested
    }
}

impl Reducible for LinkGroupStatus {
    type Action = LinkGroupStatusAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LinkGroupStatusAction::PleaseStart(links) => match *self {
                LinkGroupStatus::NotRequested => Rc::new(Self::Started { links }),
                _ => self,
            },
            LinkGroupStatusAction::Completed => match &*self {
                LinkGroupStatus::NotRequested => {
                    unreachable!("resource not requested but received completed message")
                }
                LinkGroupStatus::Completed { .. } => unreachable!(
                    "resource is already completed but received more completed message"
                ),
                LinkGroupStatus::Started { links } => Rc::new(Self::Completed {
                    links: links.clone(),
                }),
            },
        }
    }
}
