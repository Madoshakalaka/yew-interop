use yew_interop::yew;
use yew_interop_macro::declare_resources;

#[cfg(feature = "script")]
mod single_effect {
    use super::*;

    declare_resources!(
        !my_lib
        "https://cdn.com/my_lib.js"
    );
}
