use crate::static_url::static_url;
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::{HtmlCanvasElement, HtmlImageElement};

use yew_interop::declare_resources;

declare_resources!(
    toast
    "https://cdn.jsdelivr.net/npm/toastify-js@1.11.2/src/toastify.min.js"
    "https://cdn.jsdelivr.net/npm/toastify-js@1.11.2/src/toastify.min.css"
    cropper
    "https://cdnjs.cloudflare.com/ajax/libs/cropperjs/1.5.12/cropper.min.js"
    "https://cdnjs.cloudflare.com/ajax/libs/cropperjs/1.5.12/cropper.min.css"
    ! blessings
    static_url!("blessings.js")
    ! social_media_buttons
    "https://cdn.jsdelivr.net/npm/share-buttons@1.9.0/dist/share-buttons.min.js"
);



// The javascript API: https://github.com/apvarun/toastify-js/blob/572517040fae6a7f8be4a99778dacda9c933db45/README.md
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Toastify)]
    pub type Toast;

    #[wasm_bindgen(constructor, js_class = "Toastify")]
    pub fn new(config: &JsValue) -> Toast;

    #[wasm_bindgen(method, structural, js_class = "Toastify", js_name = showToast)]
    pub fn show_toast(this: &Toast);
}

pub fn show_congrats_toast() {
    let config = Object::new();
    Reflect::set(
        &config,
        &"text".into(),
        &"Yew is awesome!".to_string().into(),
    )
    .ok();
    let toast = Toast::new(&config);
    toast.show_toast();
}

// The javascript API: https://www.npmjs.com/package/cropperjs/v/1.5.12
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Cropper)]
    pub type Cropper;

    #[wasm_bindgen(constructor, js_class = "Cropper")]
    pub fn new(image: &HtmlImageElement, options: &JsValue) -> Cropper;

    #[wasm_bindgen(method, structural, js_class = "Cropper", js_name = destroy)]
    pub fn destroy(this: &Cropper);

    #[wasm_bindgen(method, structural, js_class = "Cropper", js_name = getCroppedCanvas)]
    pub fn get_cropped_canvas(this: &Cropper) -> HtmlCanvasElement;
}
