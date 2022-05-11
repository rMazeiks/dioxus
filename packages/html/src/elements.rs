use crate::{GlobalAttributes, SvgAttributes};
use dioxus_core::*;
use std::fmt::Arguments;

macro_rules! builder_constructors {
    (
        $(
            $(#[$attr:meta])*
            $name:ident {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident: $vil:ident,
                )*
            };
         )*
    ) => {
        $(
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct $name;

            impl DioxusElement for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const NAME_SPACE: Option<&'static str> = None;
            }

            impl GlobalAttributes for $name {}

            impl $name {
                $(
                    $(#[$attr_method])*
                    pub fn $fil<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
                        cx.attr(stringify!($fil), val, None, false)
                    }
                )*
            }
        )*
    };

    ( $(
        $(#[$attr:meta])*
        $name:ident <> $namespace:tt {
            $($fil:ident: $vil:ident,)*
        };
    )* ) => {
        $(
            #[allow(non_camel_case_types)]
            $(#[$attr])*
            pub struct $name;

            impl DioxusElement for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const NAME_SPACE: Option<&'static str> = Some($namespace);
            }

            impl SvgAttributes for $name {}

            impl $name {
                $(
                    pub fn $fil<'a>(&self, cx: NodeFactory<'a>, val: Arguments) -> Attribute<'a> {
                        cx.attr(stringify!($fil), val, Some(stringify!($namespace)), false)
                    }
                )*
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
