#[cfg(feature = "desktop")]
pub use dioxus_desktop as desktop;

pub mod prelude {
    pub use dioxus_core::prelude::*;
    pub use dioxus_core_macro::{format_args_f, inline_props, rsx, Props};
    pub use dioxus_html as dioxus_elements;
}
