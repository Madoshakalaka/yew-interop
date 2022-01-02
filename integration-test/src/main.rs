// remove me on release
#[cfg(feature = "yew-stable")]
extern crate yew_19 as yew;

// remove me on release
#[cfg(feature = "yew-next")]
extern crate yew_master as yew;

mod compile {
    #[cfg(feature = "script")]
    mod script_complex_example {
        #[allow(dead_code)]
        const MY_SCRIPT: &str = "https://cdn.com/my_script.js";
        #[allow(dead_code)]
        const MY_OTHER_SCRIPT: &str = "https://cdn.com/my_other_script.js";
        yew_interop::declare_resources!(
            ! my_script
            "https://a.com/b.js?timestamp=1"
            ! lib_2
            concat!("https://a.com/", "b.js")
            !lib_3
            MY_SCRIPT
            my_lib
            "https://test.com/aa.js"
            his_lib
            js concat!("/", "b.js")
            css "/a/b.min.css?mine=true"
            css "/a.css"
            js MY_OTHER_SCRIPT
        );
    }

    mod complex_example {
        #[allow(dead_code)]
        const MY_SCRIPT: &str = "https://cdn.com/my_script.js";
        yew_interop::declare_resources!(
            my_lib
            "https://test.com/aa.js"
            his_lib
            js concat!("/", "b.js")
            css "/a/b.min.css?mine=true"
            css "/a.css"
            js MY_SCRIPT
        );
    }
}

fn main() {
    println!("Done!");
}
