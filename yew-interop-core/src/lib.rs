use std::borrow::Cow;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum LinkType {
    Css,
    Js,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Link {
    pub r#type: LinkType,
    pub src: Cow<'static, str>,
}
