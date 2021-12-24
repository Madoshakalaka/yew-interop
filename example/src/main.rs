use js_sys::{Object, Reflect};
use std::rc::Rc;
mod fontawesome;
mod interop;
use fontawesome::FontawesomeSvg;

use interop::{Cropper, ResourceProvider};
use stylist::yew::{styled_component, Global};
use web_sys::HtmlImageElement;
use yew::prelude::*;

#[function_component(BlessingsExample)]
pub fn blessings_example() -> Html {
    yew_interop::use_script_effect("/static/blessings.js");
    html! {<p>{"Now check your console!"}</p>}
}

#[styled_component(SocialMediaButtonsExample)]
pub fn social_media_buttons_example() -> Html {
    #[derive(PartialEq)]
    enum State {
        Show,
        Hide,
    }
    struct Toggle;
    impl Reducible for State {
        type Action = Toggle;
        fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
            match &*self {
                State::Show => Rc::new(Self::Hide),
                State::Hide => Rc::new(Self::Show),
            }
        }
    }
    let show_buttons = use_reducer(|| State::Hide);

    // only run script when the buttons are shown
    yew_interop::use_conditional_script_effect(
        "https://cdn.jsdelivr.net/npm/share-buttons@1.9.0/dist/share-buttons.min.js",
        |s| matches!(**s, State::Show),
        show_buttons.clone(),
    );

    let class = classes!(
        css!(
            r#"
        display: grid;
        grid-auto-flow: column;
        justify-content: start;
        grid-gap: 8px;
        margin-top: 8px;
        margin-bottom: 8px;
        
        a{
            cursor: pointer;
            border-radius: 5px;
            font-size: 15px;
            padding-left: 20px;
            padding-right: 20px;
            padding-top: 5px;
            padding-bottom: 5px;
            display: inline-grid;
            grid-auto-flow: column;
            white-space: nowrap;
            margin: 10px 0px;
        }
        
        a>svg{
            color: white;
            margin-right: 10px;
            align-self:center;
            width: 20px;
        }
        a>span{
            color: white;
            font-weight: 900;
        }
        
        .btn-vk{
            background: #2683ED;
        }
        .btn-facebook{
            background: #4868AD;
        }
        .btn-twitter{
            background: #69ace8;
        }
        
    "#
        ),
        "share-btn"
    );

    let dispatcher = show_buttons.dispatcher();
    html! {
        <>

        <button onclick={move |_| dispatcher.dispatch(Toggle)}>{"Toggle"}</button>

        if matches!(*show_buttons, State::Show){
            <div {class}>
                <a class="btn-vk" data-id="vk">
                    <FontawesomeSvg view_box="0 0 576 512" d="M545 117.7c3.7-12.5 0-21.7-17.8-21.7h-58.9c-15 0-21.9 7.9-25.6 16.7 0 0-30 73.1-72.4 120.5-13.7 13.7-20 18.1-27.5 18.1-3.7 0-9.4-4.4-9.4-16.9V117.7c0-15-4.2-21.7-16.6-21.7h-92.6c-9.4 0-15 7-15 13.5 0 14.2 21.2 17.5 23.4 57.5v86.8c0 19-3.4 22.5-10.9 22.5-20 0-68.6-73.4-97.4-157.4-5.8-16.3-11.5-22.9-26.6-22.9H38.8c-16.8 0-20.2 7.9-20.2 16.7 0 15.6 20 93.1 93.1 195.5C160.4 378.1 229 416 291.4 416c37.5 0 42.1-8.4 42.1-22.9 0-66.8-3.4-73.1 15.4-73.1 8.7 0 23.7 4.4 58.7 38.1 40 40 46.6 57.9 69 57.9h58.9c16.8 0 25.3-8.4 20.4-25-11.2-34.9-86.9-106.7-90.3-111.5-8.7-11.2-6.2-16.2 0-26.2.1-.1 72-101.3 79.4-135.6z"/>
                    <span>{"VK"}</span>
                </a>
                <a class="btn-facebook" data-id="fb">
                    <FontawesomeSvg view_box="0 0 512 512" d="M504 256C504 119 393 8 256 8S8 119 8 256c0 123.78 90.69 226.38 209.25 245V327.69h-63V256h63v-54.64c0-62.15 37-96.48 93.67-96.48 27.14 0 55.52 4.84 55.52 4.84v61h-31.28c-30.8 0-40.41 19.12-40.41 38.73V256h68.78l-11 71.69h-57.78V501C413.31 482.38 504 379.78 504 256z"/>
                    <span>{"Facebook"}</span>
                </a>
                <a class="btn-twitter" data-id="tw">
                    <FontawesomeSvg view_box="0 0 512 512" d="M459.37 151.716c.325 4.548.325 9.097.325 13.645 0 138.72-105.583 298.558-298.558 298.558-59.452 0-114.68-17.219-161.137-47.106 8.447.974 16.568 1.299 25.34 1.299 49.055 0 94.213-16.568 130.274-44.832-46.132-.975-84.792-31.188-98.112-72.772 6.498.974 12.995 1.624 19.818 1.624 9.421 0 18.843-1.3 27.614-3.573-48.081-9.747-84.143-51.98-84.143-102.985v-1.299c13.969 7.797 30.214 12.67 47.431 13.319-28.264-18.843-46.781-51.005-46.781-87.391 0-19.492 5.197-37.36 14.294-52.954 51.655 63.675 129.3 105.258 216.365 109.807-1.624-7.797-2.599-15.918-2.599-24.04 0-57.828 46.782-104.934 104.934-104.934 30.213 0 57.502 12.67 76.67 33.137 23.715-4.548 46.456-13.32 66.599-25.34-7.798 24.366-24.366 44.833-46.132 57.827 21.117-2.273 41.584-8.122 60.426-16.243-14.292 20.791-32.161 39.308-52.628 54.253z"/>
                    <span>{"Twitter"}</span>
                </a>
            </div>

        }

        </>
    }
}

#[function_component(ToastExample)]
pub fn toast_example() -> Html {
    let toast_ready = interop::use_toast();
    html! {
        <>
            if toast_ready {
                <p>{"toast.js and toast.css loaded!!"}</p>
                <button onclick={|_|interop::show_congrats_toast()}>{"Send a toast"}</button>
            }else{
                <Progress id="toast-progress" label="toast.js and toast.css are loading"/>
            }
        </>
    }
}

#[styled_component(CropperExample)]
pub fn cropper_example() -> Html {
    let cropper_ready = interop::use_cropper();
    let cropper: UseStateHandle<Option<Cropper>> = use_state(|| None);
    let cropped_image_src: UseStateHandle<Option<String>> = use_state(|| None);
    let image_ref = NodeRef::default();

    let crop_square_button = {
        let cropper = cropper.clone();
        let image_ref = image_ref.clone();
        let onclick = move |_| {
            let img: HtmlImageElement = image_ref.cast().unwrap();
            let options = Object::new();
            Reflect::set(&options, &"aspectRatio".into(), &2f32.into()).ok();
            let c = Cropper::new(&img, &options);
            cropper.set(Some(c))
        };
        html_nested!(
            <button {onclick}>{"Crop a 2:1 rectangle"}</button>
        )
    };

    let destroy_cropper = |cropper: &UseStateHandle<Option<Cropper>>| {
        if let Some(c) = cropper.as_ref() {
            c.destroy();
        }
        cropper.set(None);
    };

    let confirm_button = {
        let cropper = cropper.clone();
        let cropped_image_src = cropped_image_src.clone();
        let onclick = move |_| {
            let canvas = cropper.as_ref().map(|c| c.get_cropped_canvas()).unwrap();
            let data_url = canvas.to_data_url().unwrap();
            cropped_image_src.set(Some(data_url));
            destroy_cropper(&cropper);
        };

        html_nested!(
            <button {onclick}>{"Confirm"}</button>
        )
    };

    let destroy_cropper_button = {
        let cropper = cropper.clone();
        html_nested!(
            <button onclick={move |_| destroy_cropper(&cropper)}>{"Cancel"}</button>
        )
    };

    let button_row = css!(
        r#"
        display: grid;
        grid-auto-flow: column;
        grid-gap: 8px;
        margin-bottom: 8px;
        justify-content: start;
    "#
    );

    html! {
        <>
            if cropper_ready {
                <p>{"cropper.js and cropper.css loaded!!"}</p>
                <div class={button_row}>
                    if cropper.is_some(){
                        {confirm_button}
                        {destroy_cropper_button}
                    }else{
                        {crop_square_button}
                    }
                </div>

            }else{
                <Progress id="cropper-progress" label="cropper.js and cropper.css are loading"/>
            }
            {
                cropped_image_src.as_ref().map(|src|
                html_nested!(<img class={css!("width: 10rem; height: 5rem;")} src={src.clone()}/>)
            ).unwrap_or_default()
            }

            <div>
                <img ref={image_ref} src="/static/yew-logo.png" class={css!("width: 10rem; height: 10rem; display: block; max-width: 100%;")}/>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProgressProps {
    id: &'static str,
    label: &'static str,
}

#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    html! {
        <>
            <label for={props.id}>{props.label}</label><br/>
            <progress id={props.id}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ExampleSectionProps {
    title: &'static str,
    children: Children,
}

/// just to beautify things
#[styled_component(ExampleSection)]
pub fn example_section(props: &ExampleSectionProps) -> Html {
    html! {
        <section class={css!("border: 4px dashed gray; padding: 0 8px 1.25rem 16px; border-radius: 8px;")}>
            <h1>{props.title}</h1>
            {for props.children.iter()}
        </section>
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let show_toast_example = use_state(|| false);
    let show_cropper_example = use_state(|| false);
    let show_blessings_example = use_state(|| false);
    let show_social_media_buttons_example = use_state(|| false);

    let github_link = css!(
        r#"
        svg{
            color: black;
            height: 1rem;
        }
    "#
    );

    let examples = css!(
        r#"
            display: grid;
            justify-content: center;
            grid-gap: 32px;
            margin-bottom: 2rem;
        "#
    );

    html! {
        <>

            <ResourceProvider>
                <Global css=r#"
                    body{
                        display: grid;
                        justify-content: center;
                    }
                "#/>


                <h1 style="justify-self: start;">{"yew-interop demo"}</h1>

                <div class={examples}>
                    <ExampleSection title="Toast Example">
                        <p><small>{"using "} <code>{"yew_interop::declare_resources!"}</code> {" and "}
                            <a target="_blank" rel="noopener" href="https://github.com/apvarun/toastify-js/releases/tag/1.11.2">{"toastify-js 1.11.2"}</a>
                        </small></p>

                        if *show_toast_example{
                            <ToastExample/>
                        }else{
                            <button onclick={move |_| show_toast_example.set(true)}>
                                {"Load toast.js and toast.css asynchronously"}
                            </button>
                        }
                    </ExampleSection>

                    <ExampleSection title="Image-cropper Example">
                        <p><small>{"using "} <code>{"yew_interop::declare_resources!"}</code> {" and "}
                            <a target="_blank" rel="noopener" href="https://www.npmjs.com/package/cropperjs/v/1.5.12">{"Cropper.js 1.5.12"}</a>
                        </small></p>

                        if *show_cropper_example{
                            <CropperExample/>
                        }else{
                            <button onclick={move |_| show_cropper_example.set(true)}>
                                {"Load cropper.js and cropper.css asynchronously"}
                            </button>
                        }
                    </ExampleSection>

                    <ExampleSection title="Blessings Example">
                        <p><small>{"using "} <code>{"yew_interop::use_script_effect"}</code> {" and "}
                            <a target="_blank" rel="noopener" href="/static/blessings.js">{"blessings.js"}</a>
                        </small></p>

                        if *show_blessings_example{
                            <BlessingsExample/>
                        }else{
                            <button onclick={move |_| show_blessings_example.set(true)}>
                                {"Run blessings.js"}
                            </button>
                        }
                    </ExampleSection>

                    <ExampleSection title="Social Media Buttons Example">
                        <p><small>{"using "} <code>{"yew_interop::use_conditional_script_effect"}</code> {" and "}
                            <a target="_blank" rel="noopener" href="https://www.npmjs.com/package/share-buttons/v/1.9.0">{"share-buttons 1.9.0"}</a>
                        </small></p>

                        if *show_social_media_buttons_example{
                            <SocialMediaButtonsExample/>
                        }else{
                            <button onclick={move |_| show_social_media_buttons_example.set(true)}>
                                {"Run share-buttons.js"}
                            </button>
                        }
                    </ExampleSection>
                </div>

                <a class={github_link} target="_blank" rel="noopener" href="https://github.com/Madoshakalaka/yew-interop/tree/master/example">{"source code for this page "}
                    <FontawesomeSvg view_box="0 0 496 512" d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/>
                </a>

                <p><small>{"svg are provided by " }
                <a target="_blank" rel="noopener" href="https://fontawesome.com/license">{"fontawesome"}</a>
                {" and unmodified"}</small></p>

            </ResourceProvider>
        </>
    }
}

fn main() {
    yew::set_event_bubbling(false);
    yew::start_app::<App>();
}
