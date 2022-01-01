<div align="center">
<h1>Yew Interop</h1>
<img alt="Crates.io" src="https://img.shields.io/crates/v/yew-interop">
<a href="https://madoshakalaka.github.io/yew-interop/v0.2.1"><img alt="demo badge" src="https://img.shields.io/badge/demo%20v0.2.1-up-brightgreen"/></a>
<a href="https://madoshakalaka.github.io/yew-interop/master"><img alt="demo badge" src="https://img.shields.io/badge/demo%20master-up-brightgreen"/></a>
</div>

## Load On Demand

With `yew-interop`, each resource is requested on demand when a consuming component requests it.

If you include your libraries using the
[JS-snippet method with wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html)
or insert them as `<script/>` or `<link/>` directly in the `index.html`,
the resources will load for every request,
even if there is no consuming component.
This can cause congestion and wasted data.


## Load Once, Use Everywhere

Each resource is strictly requested once.
If a resource has been requested in one component,
other consuming components won't trigger a reload.
Other requests to the same resource will either wait for the load to complete,
or return ready immediately if the resource is loaded.


## Demo

[The example folder](https://github.com/Madoshakalaka/yew-interop/tree/master/example) 
has a demo website built with`yew-interop`

The gif below shows the first two use cases,
loading speed is throttled for demo purposes.

![yew interop demo gif](https://madoshakalaka.github.io/yew-interop/master/static/yew-interop-demo.gif)

[Check out the full online demo](https://madoshakalaka.github.io/yew-interop/master)

# Install

Master branch is the in-development branch,

```toml
yew-interop = {git="https://github.com/Madoshakalaka/yew-interop.git", branch="master", features=["yew-stable"]}
```

If you are using yew-next (yew's master branch), change the `yew-stable` feature to `yew-next`.

Or you can install the latest version published on crates.io, which uses yew 0.19.

```toml
yew-interop = "0.2"
```

Note the `yew-next`/`yew-stable` features only exist in the master branch
since published crates can't have git dependencies.

# API

## Asynchronously Load CSS or Javascript Libraries

If your javascript library exposes functions or objects you want to use in Rust,
then `yew_interop::declare_resources!` is the right choice.

First you want to create a separate module `interop.rs` and declare your dependencies there.

```rust
// interop.rs
use yew_interop::declare_resources;

declare_resources!(
    library_a
    "https://my-cdn.com/library-a.min.js"
    library_b
    "https://my-cdn.com/library-b.min.js"
    "https://my-cdn.com/library-b.min.css"
);
```

This macro expands into a `<ResourceProvider/>` component.
you want to wrap your application in the provider:

```rust
// main.rs
mod interop;
use interop::ResourceProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ResourceProvider>
            // the rest of your app
        </ResourceProvider>
    }
}
```

The macro will also expand into hooks by prepending your resource names with "_use__", in this case,
the macro will expand into `pub fn use_library_a() -> bool` and `pub fn use_library_b() -> bool`

At your consuming component, you can use these hooks to asynchronously wait for libraries to be loaded:

```rust
use crate::interop::use_library_a;

#[function_component(Consumer)]
pub fn consumer() -> Html {
    let library_a_ready = use_library_a();

    html! {
        if library_a_ready{
            // use library a here
        }else{
            <p>"please wait..."</p>
        }
    }
}
```

>For javascript libraries,
you will also need to write some stubs using `wasm-bindgen` and `js-sys` before using the library in Rust.
The wasm-bindgen book has [a good chapter](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html) on that.
You can also check out our demo website and have a look [how it's done there](https://github.com/Madoshakalaka/yew-interop/blob/master/example/src/interop.rs)

## Explicit Resource Type

The `declare_resources!` macro needs to know whether a url is JavaScript or CSS.
When you provide a string literal as in the examples above,
the macro derives the information from the suffix of the last path segment (either .js or .css).
When the string literal doesn't end with .js or .css,
or when you provide other expressions like a macro call or an identifier,
you need to manually specify the URL type by prepending the custom keyword js/css
before the url.

`declare_resources!` will accept any expression with a return type that implements `Into<Cow<'static, str>>`,
so `&'static str`, `String`, `Cow<'static, str>` are all fine.

here's a more complex example:

```rust
const MY_LIB_JS: &'static str = "https://cdn.com/my_lib.js";

declare_resources!(
        my_lib
        js MY_LIB_JS
        "https://cdn.com/my_lic_b.css" // <-- when string literal is provided, script type is determined from the suffix
        js concat!("https://a.com/", "b.js")
        my_lib_b
        js static_url!("my_lib_b.js")
        css "https://somehow.ends/with/.js" // explicit type css overrides the suffix
        my_lib_c
        js String::from("https://a.com/test.js")
    );

```

## Side Effect Javascript

Here, side effect scripts refers to the JavaScript that run something onload,
as opposed to a library that exposes functions and classes.

If your javascript is a side effect script, 
you want to enable the `script` feature.


```toml
yew-interop = {git="https://github.com/Madoshakalaka/yew-interop.git",  features=["yew-stable", "script"]}
```
or
```toml
yew-interop = {version = "0.2", features = ["script"]}
```

You will need to prepend the identifier of a script with an exclamation mark (!).
And only one script url for each identifier, here's an example:

```rust
// interop.rs

declare_resources!(
    ! my_script // <- exclamation mark for side effect scripts
    "https://cdn.com/script.js"
    lib // <- normal library
    "https://cdn.com/lib.js"
    "https://cdn.com/lib.css"
);
```

You never need to specify the resource type explicitly, 
since only JavaScript is allowed.

Same as previous examples, this will expand into a `use_<identifier>` hook.
What's different is that instead of a bool,
the hook returns an `Option<Script>`,
it is none when the script is loading.

To run the script, you will need to render a `<ScriptEffect/>` component.
This allows you to freely control whether and when the script should be run.
The `<ScriptEffect/>` component is a [portal](https://yew.rs/docs/next/advanced-topics/portals)
to the `<head/>` element of the document,
so it won't render anything in its place,
it will only run the script on render.

```rust
use yew_interop::ScriptEffect;

/// this example simply runs the script on every re-render, if the script is ready.
#[function_component(App)]
pub fn app() -> Html {
    let script = use_my_script();
    
    // ...snip
    
    html! {
        if my_script.is_none(){
            <p>{"Please wait..."}</p>
        }else{
            <p>{"Script Completed!"}</p>
            <ScriptEffect {script}>
        }
    }
}
```

If your script depends on other components being rendered,
such as the fourth example [in the demo](https://madoshakalaka.github.io/yew-interop/master/static/yew-interop-demo.gif),
where the script adds onclick handlers to the rendered elements,
you will need to guarantee the script is rendered after all the dependees.

Yew renders descendents in a breadth-first order from bottom to top,
which is not the most intuitive rendering order.

One way to guarantee the correct rendering order is to
place the `<ScriptEffect/>` component as a **sibling on top of the deepest dependees**,

For example, let's say your script depends on two components `<ComponentA/>` and `<ComponentB/>`.

The case below shows a correct placement where A and B has the same depth,
the rendering order here is B -> A -> ScriptEffect

```rust
html!{
    <>
    <ScriptEffect {script}/>
    <ComponentA/>
    <ComponentB/>
    // <ScriptEffect {script}> !!! do not place here, otherwise it would render first
    </>
}
```

Here's trickier one, where B is deeper, so we place our component on top of B:

```rust
html!{
    <>
    <ComponentA/>
    <Container>
        <ScriptEffect {script}>
        <ComponentB/>
    </Container>
    </>
}
```

The rendering order is Container -> A -> B -> ScriptEffect.
