use crate::{GlobalAttributes, SvgAttributes};
use dioxus_core::*;
use std::fmt::Arguments;

macro_rules! builder_constructors {
    (
        $(
            $name:ident {
            };
         )*
    ) => {
        $(
            pub struct $name;

            impl DioxusElement for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const NAME_SPACE: Option<&'static str> = None;
            }
        )*
    };
}

builder_constructors! {
    input {
    };
}
impl input {
    pub fn r#type<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
        loop {}
    }
}
