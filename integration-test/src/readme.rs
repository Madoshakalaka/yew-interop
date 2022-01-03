#[allow(dead_code)]
mod line75to86 {
    // interop.rs
    use yew_interop::declare_resources;

    declare_resources!(
        library_a
        "https://my-cdn.com/library-a.min.js"
        library_b
        "https://my-cdn.com/library-b.min.js"
        "https://my-cdn.com/library-b.min.css"
    );
}

#[allow(dead_code)]
mod line150to166 {
    use yew_interop::declare_resources;

    const MY_LIB_JS: &str = "https://cdn.com/my_lib.js";

    declare_resources!(
        my_lib
        js MY_LIB_JS
        "https://cdn.com/my_lic_b.css" // <-- when a string literal is provided, script type is determined from the suffix
        js concat!("https://a.com/", "b.js")
        my_lib_b
        css "/somehow/ends/with/.js" // <-- explicit type css overrides the suffix
        my_lib_c
        js String::from("https://a.com/test.js")
    );
}

#[cfg(feature = "script")]
#[allow(dead_code)]
mod line189to201 {
    // This example requires the script feature
    // file: interop.rs
    use yew_interop::declare_resources;

    declare_resources!(
        lib // <- normal library
        "https://cdn.com/lib.js"
        "https://cdn.com/lib.css"
        ! my_script // <- exclamation mark for side effect scripts
        "https://cdn.com/script.js"
    );
}

#[cfg(feature = "script")]
#[allow(dead_code)]
mod line218to250 {
    // This example requires the script feature

    // file: interop.rs
    use yew_interop::declare_resources;

    declare_resources!(
        ! my_script // <- exclamation mark for side effect scripts
        "https://cdn.com/script.js"
    );

    // consuming file:
    use yew::prelude::*;
    use yew_interop::ScriptEffect;

    /// this example simply runs the script on every re-render, if the script is ready.
    #[function_component(MyComp)]
    pub fn my_comp() -> Html {
        let script = use_my_script();

        // ...snip

        html! {
            if script.is_none(){
                <p>{"Please wait..."}</p>
            }else{
                <p>{"Script Completed!"}</p>
                <ScriptEffect {script}/>
            }
        }
    }
}
