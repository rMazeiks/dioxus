use dioxus_core::*;
use std::fmt::Arguments;

pub struct input;

impl DioxusElement for input {
    const TAG_NAME: &'static str = "input";
    const NAME_SPACE: Option<&'static str> = None;
}

impl input {
    pub fn r#type(&self) -> VNode {
        loop {}
    }
}
