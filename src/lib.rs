#[cfg(feature = "desktop")]
pub use dioxus_desktop as desktop;

pub mod prelude {
    pub use dioxus_core::prelude::*;
    pub use dioxus_html as dioxus_elements;
}
