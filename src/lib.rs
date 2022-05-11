pub mod dioxus_html {
    mod elements {
        pub struct input;

        impl input {
            pub fn r#type(&self) {
                loop {}
            }
        }
    }
    pub mod input {}

    pub use elements::*;
}
