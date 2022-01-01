/// production/development aware static url
#[cfg(debug_assertions)]
macro_rules! static_url {
    ($rest:tt) => {
        concat!("/static/", $rest)
    };
}
#[cfg(not(debug_assertions))]
macro_rules! static_url {
    ($rest:tt) => {
        concat!(
            "/yew-interop/",
            env!("YEW_INTEROP_DEMO_VERSION"),
            "/static/",
            $rest
        )
    };
}
pub(crate) use static_url;
