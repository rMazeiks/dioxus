mod inner {
    pub struct conflict;

    impl conflict {
        pub fn do_stuff(&self) {}
    }
}
mod conflict {}

pub use inner::*;
