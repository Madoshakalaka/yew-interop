use yew::prelude::*;

// go to https://fontawesome.com/icons/
// download the desired svg
// Create a new component using <FontawesomeSvg> with the `viewBox` attribute and the `d` attribute
// inside the <path>

// tips: svg color is changeable via css, just select the svg and set color as usual

#[derive(Properties, PartialEq, Clone)]
pub struct FontawesomeSvgProps {
    pub view_box: &'static str,
    pub d: &'static str,



}

#[function_component(FontawesomeSvg)]
pub fn fontawesome_svg(props: &FontawesomeSvgProps) -> Html {
    html! {
        <svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox={props.view_box}>
            <path fill="currentColor" d={props.d}/>
        </svg>
    }
}
