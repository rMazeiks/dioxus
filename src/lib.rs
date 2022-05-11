mod inner {
    pub struct conflict;

    impl conflict {
        pub fn r#type(&self) {
            loop {}
        }
    }
}
pub mod conflict {}

pub use inner::*;
