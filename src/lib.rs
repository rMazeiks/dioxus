mod inner {
    pub struct conflict;

    impl conflict {
        pub fn do_stuff(&self) {}
    }
}
pub mod conflict {}

pub use inner::*;
