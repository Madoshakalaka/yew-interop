//! [check out the guide on github](https://github.com/Madoshakalaka/yew-interop/tree/v0.2.1)

#[cfg(feature = "script")]
pub mod script;

#[cfg(feature = "script")]
pub use script::ScriptEffect;

use std::rc::Rc;

// remove me on release
#[cfg(all(feature = "yew-stable", feature = "yew-next"))]
compile_error!(
    "feature \"yew-stable\" and feature \"yew-next\" cannot be enabled at the same time"
);
// remove me on release
#[cfg(not(any(feature = "yew-stable", feature = "yew-next")))]
compile_error!("one of feature \"yew-stable\" and feature \"yew-next\" has to be enabled");

// remove me on release
#[cfg(feature = "yew-stable")]
extern crate yew_19 as yew;

// remove me on release
#[cfg(feature = "yew-next")]
extern crate yew_master as yew;

use yew::Reducible;

#[doc(hidden)]
pub use yew_interop_core::{Link, LinkType};
pub use yew_interop_macro::declare_resources;

#[doc(hidden)]
pub enum LinkGroupStatusAction {
    PleaseStart(Vec<Link>),

    Completed,
}

#[doc(hidden)]
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
