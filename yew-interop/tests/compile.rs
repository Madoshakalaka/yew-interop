use yew_interop_macro::declare_resources;

mod single_lib {
    use super::*;

    declare_resources!(
        my_lib
        "https://cdn.com/my_lib.js"
    );
}

mod two_libs {
    use super::*;

    declare_resources!(
        my_lib
        "https://cdn.com/my_lib.js"
        my_lib_b
        "https://cdn.com/my_lib_b.js"
        "https://cdn.com/my_lic_b.css"
    );
}

mod complex_libs {
    use super::*;

    #[allow(dead_code)]
    const MY_LIB_JS: &'static str = "https://cdn.com/my_lib.js";

    declare_resources!(
        my_lib
        js MY_LIB_JS
        css "https://somehow.ends/with/.js"
        js concat!("https://a.com/", "b.js")
        my_lib_b
        "https://cdn.com/my_lib_b.js"
        "https://cdn.com/my_lic_b.css"
        my_lib_c
        js String::from("https://a.com/test.js")
    );
}
