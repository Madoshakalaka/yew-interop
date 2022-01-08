use js_sys::JsString;
use std::borrow::Cow;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use yew::{create_portal, function_component, html, Properties, Reducible};

#[doc(hidden)]
pub use wasm_bindgen_futures;

#[doc(hidden)]
pub async fn fetch_script(url: Cow<'static, str>) -> String {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text: JsString = JsFuture::from(resp.text().unwrap())
        .await
        .unwrap()
        .unchecked_into();
    text.into()
}

#[doc(hidden)]
pub enum ScriptLoader {
    NotRequested,
    Started,
    Completed(Script),
}

impl Default for ScriptLoader {
    fn default() -> Self {
        Self::NotRequested
    }
}

#[doc(hidden)]
pub enum ScriptLoaderAction {
    Start,
    Finish(Rc<String>),
}

impl Reducible for ScriptLoader {
    type Action = ScriptLoaderAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ScriptLoaderAction::Start => match *self {
                ScriptLoader::NotRequested => Rc::new(Self::Started),
                ScriptLoader::Started => unreachable!(
                    "script already started to load, can't receive another start request"
                ),
                ScriptLoader::Completed(_) => unreachable!(
                    "script load already completed, can't receive another start request"
                ),
            },
            ScriptLoaderAction::Finish(content) => match *self {
                ScriptLoader::NotRequested => {
                    unreachable!("script finished load, request should have started")
                }
                ScriptLoader::Started => Rc::new(Self::Completed(Script { content })),
                ScriptLoader::Completed(_) => {
                    unreachable!("script completed load, can't receive another complete request")
                }
            },
        }
    }
}

/// A JavaScript cached globally
#[derive(Clone, PartialEq)]
pub struct Script {
    content: Rc<String>,
}

#[cfg_attr(documenting, doc(cfg(feature = "script")))]
#[derive(Properties, PartialEq)]
pub struct ScriptEffectProps {
    pub script: Script,
}

#[cfg_attr(documenting, doc(cfg(feature = "script")))]
/// This component runs javascript __on render__.
/// Note this is different from the [`yew::use_effect`] hook, which runs __after rendering__.
#[function_component(ScriptEffect)]
pub fn script_effect(props: &ScriptEffectProps) -> Html {
    create_portal(
        html! {
            <script type="text/javascript">
                {props.script.content.clone()}
            </script>
        },
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .head()
            .unwrap()
            .into(),
    )
}
