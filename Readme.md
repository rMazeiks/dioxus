This is a minimal (i think) example of a crate that makes Cargo panic.

lib.rs:

```rust
mod inner {
    pub struct conflict;

    impl conflict {
        pub fn do_stuff(&self) {}
    }
}
mod conflict {}

pub use inner::*;
```

examples/example.rs
```rust
fn main() {
    dioxus::conflict.do_stuff();
}
```

Test or run the example to get the panic:
```
# cargo panics:
cargo test

# or
cargo run --example example
```