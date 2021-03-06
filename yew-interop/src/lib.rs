// for publishing docs
#![cfg_attr(documenting, feature(doc_cfg))]

//!<div align="center"> <h1>Yew Interop</h1> <p> <strong>Load JavaScript and CSS asynchronously in Yew</strong> </p>
// fix both links on release
//!  <a href="https://crates.io/crates/yew-interop"><img alt="Crates.io" src="https://img.shields.io/crates/v/yew-interop"></a>
// fix after release
// fix on release
//!<a href="https://madoshakalaka.github.io/yew-interop/v0.3.0"><img alt="demo badge released" src="https://img.shields.io/badge/demo-v0.3.0-brightgreen"/></a>
//!<a href="https://madoshakalaka.github.io/yew-interop/master"><img alt="demo badge master" src="https://img.shields.io/badge/demo-master-brightgreen"/></a>
// fix on release
//!<a href="https://docs.rs/yew-interop"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-released-brightgreen"/></a>
//!<a href="https://madoshakalaka.github.io/yew-interop/docsrs/yew_interop/"><img alt="docs master" src="https://img.shields.io/badge/docs-master-brightgreen"/></a>
//!<a href="#"><img alt="minimal rustc" src="https://img.shields.io/badge/rustc-1.56%2B-lightgrey"/></a>
//!</div>
//!
//!## Load On Demand
//!
//!With `yew-interop`, each resource is requested on demand when a consuming component requests it.
//!
//!
//!If you include your libraries using the
//![JS-snippet method with wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html)
//!or insert them as `<script/>` or `<link/>` directly in the `index.html`,
//!the resources will load for every request,
//!even if there is no consuming component.
//!This can cause congestion and wasted data.
//!
//!
//!## Load Once, Use Everywhere
//!
//!Each resource is strictly requested once.
//!If a resource has been requested in one component,
//!other consuming components won't trigger a reload.
//!Other requests to the same resource will either wait for the load to complete,
//!or return ready immediately if the resource is loaded.
//!
//!
//!## Demo
//!
//![The example folder](https://github.com/Madoshakalaka/yew-interop/tree/master/example)
//!has a demo website built with`yew-interop`
//!
//!The gif below shows the first two use cases,
//!loading speed is throttled for demo purposes.
//!
//!![yew interop demo gif](https://madoshakalaka.github.io/yew-interop/master/static/yew-interop-demo.gif)
//!
//![Check out the full online demo](https://madoshakalaka.github.io/yew-interop/master)
//!
//!# Install
//!The master branch has the the lastest in-development code.
//!
//!```toml
//!yew-interop = {git="https://github.com/Madoshakalaka/yew-interop.git", branch="master", features=["yew-stable"]}
//!```
//!
//!The `yew-stable` feature works with the latest release of yew on crates.io, currently 0.19.
//!If you are using yew-next (yew's master branch), change the `yew-stable` feature to `yew-next`.
//!
//!Or you can install the latest version published on crates.io, which uses yew 0.19.
//!
// bump this after release when 0.x or major version changed
//!```toml
//!yew-interop = "0.3"
//!```
//!
//!Note the `yew-next`/`yew-stable` features only exist in the master branch
//!since published crates can't have git dependencies.
//!
//!# API
//!
//!## Asynchronously Load CSS or Javascript Libraries
//!
//!If your javascript library exposes functions or objects you want to use in Rust,
//!then `yew_interop::declare_resources!` is the right choice.
//!
//!First you want to create a separate module `interop` and declare your dependencies there.
//!
#![doc = "```rust"]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
//!// alternatively, create a separate file `interop.rs`,
//!// and add `mod interop;` to `main.rs` to have tidier code.
//!mod interop{
//!    use yew_interop::declare_resources;
//!
//!    declare_resources!(
//!        library_a
//!        "https://my-cdn.com/library-a.min.js"
//!        library_b
//!        "https://my-cdn.com/library-b.min.js"
//!        "https://my-cdn.com/library-b.min.css"
//!        library_c
//!        "/static/library-c.min.js"
//!        "/static/library-c.min.css"
//!    );
//!}
#![doc = "```"]
//!This macro expands into a `<ResourceProvider/>` component.
//!you want to wrap the root of your application in the provider:
//!
#![doc = "```rust"]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
//!# mod interop{
//!#    use yew_interop::declare_resources;
//!#
//!#    declare_resources!(
//!#        library_a
//!#        "https://my-cdn.com/library-a.min.js"
//!#        library_b
//!#        "https://my-cdn.com/library-b.min.js"
//!#        "https://my-cdn.com/library-b.min.css"
//!#    );
//!# }
//!use yew::prelude::*;
//!use interop::ResourceProvider;
//!
//!#[function_component(App)]
//!pub fn app() -> Html {
//!    html! {
//!        <ResourceProvider>
//!# <></>
//!            // the rest of your app
//!        </ResourceProvider>
//!    }
//!}
#![doc = "```"]
//!The macro will also expand into hooks by prepending your resource names with "_use__", in this case,
//!the macro will expand into `pub fn use_library_a() -> bool` and `pub fn use_library_b() -> bool`
//!
//!At your consuming component, you can use these hooks to asynchronously wait for libraries to be loaded:
//!
#![doc = "```rust"]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
//!# mod interop{
//!#    use yew_interop::declare_resources;
//!#
//!#    declare_resources!(
//!#        library_a
//!#        "https://my-cdn.com/library-a.min.js"
//!#        library_b
//!#        "https://my-cdn.com/library-b.min.js"
//!#        "https://my-cdn.com/library-b.min.css"
//!#    );
//!# }
//!
//!use yew::prelude::*;
//!use interop::use_library_a;
//!
//!#[function_component(Consumer)]
//!pub fn consumer() -> Html {
//!    let library_a_ready = use_library_a(); // <-- generated hook
//!
//!    html! {
//!        if library_a_ready{
//!            // use library a here
//!# {html!{}}
//!        }else{
//!            <p>{"please wait..."}</p>
//!        }
//!    }
//!}
#![doc = "```"]
//!>For javascript libraries,
//!you will also need to write some stubs using `wasm-bindgen` and `js-sys` before using the library in Rust.
//!The wasm-bindgen book has [a good chapter](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html) on that.
//!You can also check out our demo website and have a look [how it's done there](https://github.com/Madoshakalaka/yew-interop/blob/master/example/src/interop.rs)
//!
//!## Explicit Resource Type
//!
//!The `declare_resources!` macro needs to know whether a url is JavaScript or CSS.
//!When you provide a string literal as in the examples above,
//!the macro derives the information from the suffix of the last path segment.
//!It expects .js or .css and is smart enough to exclude the query params or the fragment.
//!
//!When the path segment doesn't end with .js or .css,
//!or when you provide other expressions like a macro call or an identifier,
//!you need to manually specify the URL type by prepending the custom keyword js/css
//!before the url.
//!
//!`declare_resources!` will accept any expression with a return type that implements `Into<Cow<'static, str>>`,
//!so `&'static str`, `String`, `Cow<'static, str>` are all fine.
//!
//!here's a more complex example:
//!
#![doc = "```rust"]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
//!use yew_interop::declare_resources;
//!
//!const MY_LIB_JS: &str = "https://cdn.com/my_lib.js";
//!
//!declare_resources!(
//!        my_lib
//!        js MY_LIB_JS
//!        "https://cdn.com/my_lic_b.css" // <-- when a string literal is provided, script type is determined from the suffix
//!        "/static/snippet.js"
//!        js concat!("https://a.com/", "b.js")
//!        my_lib_b
//!        css "/somehow/ends/with/.js" // <-- explicit type css overrides the suffix
//!        my_lib_c
//!        js String::from("https://a.com/test.js")
//!    );
//!
#![doc = "```"]
//!## Side Effect Javascript
//!
//!Here, side effect scripts refers to the JavaScript that run something onload,
//!as opposed to a library that exposes functions and classes.
//!
//!If your javascript is a side effect script,
//!you want to enable the `script` feature.
//!
//!
//!```toml
//!# change yew-stable to yew-next if you use yew's master branch
//!yew-interop = {git="https://github.com/Madoshakalaka/yew-interop.git",  features=["yew-stable", "script"]}
//!```
//!or
// bump this after release when 0.x or major version changed
//!```toml
//!yew-interop = {version = "0.3", features = ["script"]}
//!```
//!
//!You will need to prepend the identifier of a script with an exclamation mark (!).
//!And only one script url for each identifier, here's an example:
//!
#![cfg_attr(feature = "script", doc = "```rust")]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
#![cfg_attr(
    feature = "script",
    doc = r##"use yew_interop::declare_resources; 
    
declare_resources!(
   lib // <- normal library
   "https://cdn.com/lib.js"
   "/static/lib.css"
   ! my_script // <- exclamation mark for side effect scripts
   "https://cdn.com/script.js"
);
```
"##
)]
//!You never need to specify the resource type explicitly,
//!since only JavaScript is allowed.
//!
//!Same as previous examples, this will expand into a `use_<identifier>` hook.
//!What's different is that instead of a bool,
//!the hook returns an `Option<Script>`,
//!it is `None` when the script is loading.
//!
//!To run the script, you will need to render a `<ScriptEffect/>` component and pass
//!the script object to the component.
//!This allows you to freely control whether and when the script should be run.
//!The `<ScriptEffect/>` component is a [portal](https://yew.rs/docs/next/advanced-topics/portals)
//!to the `<head/>` element of the document,
//!so it won't render anything in its place,
//!it will only run the script on render.
#![cfg_attr(feature = "script", doc = "```rust")]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
#![cfg_attr(
    feature = "script",
    doc = r##"mod interop{
   use yew_interop::declare_resources;
   declare_resources!(
       ! my_script
       "https://cdn.com/script.js"
   );
}

use yew::prelude::*;
use yew_interop::ScriptEffect;
use interop::use_my_script; // <-- generated hook

/// this example simply runs the script on every re-render, if the script is ready.
#[function_component(MyComp)]
pub fn my_comp() -> Html {
    let script = use_my_script(); // <-- returns Option<Script>

    // ...snip

    html! {
        if let Some(script) = script{
           <ScriptEffect {script}/>
        }else{
           <p>{"Please wait..."}</p>
        }
    }
}
```
"##
)]

//!If your script depends on other components being rendered,
//!such as the fourth example [in the demo](https://madoshakalaka.github.io/yew-interop/master/),
//!where the script adds onclick handlers to the rendered elements,
//!you will need to guarantee the script is rendered after all the dependees.
//!
//!Yew renders descendents in a breadth-first order from bottom to top,
//!which is not the most intuitive rendering order.
//!
//!One way to guarantee the correct rendering order is to
//!place the `<ScriptEffect/>` component as a **sibling on top of the deepest dependees**,
//!
//!For example, let's say your script depends on two components `<ComponentA/>` and `<ComponentB/>`.
//!
//!The case below shows a correct placement where A and B has the same depth,
//!
#![cfg_attr(feature = "script", doc = "```rust")]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
#![cfg_attr(
    feature = "script",
    doc = r##"# use yew_interop::declare_resources;
#
# declare_resources!(
#    ! my_script
#    "https://cdn.com/script.js"
# );
#
# use yew::prelude::*;
# use yew_interop::ScriptEffect;
#
# #[function_component(ComponentA)]
# pub fn component_a() -> Html {
#    html! {}
# }
# #[function_component(ComponentB)]
# pub fn component_b() -> Html {
#    html! {}
# }
# #[function_component(MyComp)]
# pub fn my_comp() -> Html {
#    let script = use_my_script().unwrap();
#
   html!{
       <>
       <ScriptEffect {script}/>
       <ComponentA/>
       <ComponentB/>
       // <ScriptEffect {script}/> !!! do not place here, otherwise it would render first
       </>
   }
# }
```
"##
)]
//!the rendering order here is B -> A -> ScriptEffect.
//!
//!Here's trickier one, where B is deeper, so we place our component on top of B:
//!
#![cfg_attr(feature = "script", doc = "```rust")]
// remove this on release
#![cfg_attr(feature = "yew-stable", doc = "# extern crate yew_19 as yew;")]
// delete this on release
#![cfg_attr(feature = "yew-next", doc = "# extern crate yew_master as yew;")]
#![cfg_attr(
    feature = "script",
    doc = r##"# use yew_interop::declare_resources;
#
# declare_resources!(
#    ! my_script
#    "https://cdn.com/script.js"
# );
#
# use yew::prelude::*;
# use yew_interop::ScriptEffect;
#
# #[function_component(ComponentA)]
# pub fn component_a() -> Html {
#    html! {}
# }
# #[function_component(ComponentB)]
# pub fn component_b() -> Html {
#    html! {}
# }
# #[function_component(ComponentC)]
# pub fn component_c() -> Html {
#     html! {
#        <div></div>
#    }
# }
#[derive(Properties, PartialEq)]
pub struct ContainerProps {
   children: Children
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
   // --snip--
   html! {
       {for props.children.iter()}
   }
}

# #[function_component(MyComp)]
# pub fn my_comp() -> Html {
#    let script = use_my_script().unwrap();
#
html!{
    <>
    <ComponentA/>
    <Container>
        <ScriptEffect {script}/>
        <ComponentB/>
    </Container>
    <ComponentC/>
    </>
}
# }
```
"##
)]
//!The rendering order is C -> Container -> A -> B -> ScriptEffect.
//!
//!# Contributing
//!
//!Your pull request is welcome!
//!There is extensive testing in CI.
//!Be sure to check out our [development guide](https://github.com/Madoshakalaka/yew-interop/blob/master/CONTRIBUTING.md).
//!
#[doc(hidden)]
#[cfg(feature = "script")]
pub mod script;

#[cfg(feature = "script")]
#[doc(inline)]
pub use script::ScriptEffect;

#[cfg(feature = "script")]
#[doc(inline)]
pub use script::ScriptEffectProps;
use std::borrow::Cow;

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
pub use yew_interop_core::LinkType;
pub use yew_interop_macro::declare_resources;

#[doc(hidden)]
#[derive(PartialEq, Debug, Clone)]
pub struct Link {
    pub r#type: LinkType,
    pub src: Cow<'static, str>,
}

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
