# yew-interop

## Demo

The example folder has a demo website built with`yew-interop`,
the animation below shows the first two use cases.
To see a full example of every use case,
cd into example and run `trunk serve`.

(loading speed is throttled in the animation for demo purposes)

![yew interop demo](https://raw.githubusercontent.com/Madoshakalaka/warehouse/master/images/yew-interop-demo.gif)

# Install

This branch works with yew's `master` branch.

To install, add this in your `Cargo.toml`

```toml
yew-interop = {git="https://github.com/Madoshakalaka/yew-interop.git", branch="master"}
```

If you are using yew 0.19, use v0.1 instead.

```toml
yew-interop = "0.1"
```

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

This macro expands into a `<ResourceProvider/>` component, you want to wrap your application in the provider:

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
You can also run the `example` website and have a look [how it's done there](https://github.com/Madoshakalaka/yew-interop/blob/master/example/src/interop.rs)


### Load On Demand

With `yew-interop`, each resource is requested on demand when a consuming component requests it.

If you include your libraries using the
[JS-snippet method with wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html)
or insert them as `<script/>` or `<link/>` directly in the `index.html`. Then the resources will be loaded
for every request, even if there is no consuming component. This can cause congestion and
waste data.


### Load Once, Use Everywhere

If a resource has been requested in one component,
other consuming components won't trigger a reload.
Subsequent hook calls will return `true` immediately.

## Side Effect Javascript

If your javascript is a side effect script, you can use the `yew_interop::use_script_effect()` hook.
The hook will run the script every time your component finishes rendering.

```rust
use yew_interop::use_script_effect;

use_script_effect("https://my-cdn.com/my-script.js");
```

The script will also asynchronously load,
so expect the first execution to have a delay,
the browser will cache the script so subsequent execution will be fast.

If you only want to re-run the script conditionally, use the `yew_interop::use_conditional_script_effect()` hook.

```rust
use yew_interop::use_conditional_script_effect;

use_conditional_script_effect(
"https://my-cdn.com/my-script.js",
|dep| {
//the script runs only when this function returns true
},
some_dep);
```

[The example crate](https://github.com/Madoshakalaka/yew-interop/tree/master/example) has demos for both hooks, run `trunk serve` and play around!
