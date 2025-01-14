//! Example: Memoization
//! --------------------
//!
//! This example showcases how memoization works in Dioxus.
//!
//! Memoization is the process in which Dioxus skips diffing child components if their props don't change.
//! In React, components are never memoized unless wrapped in `memo` or configured with `shouldComponentUpdate`.
//!
//! Due to the safety guarantees of Rust, we can automatically memoize components in some circumstances. Whenever a
//! component's properties are valid for the `'static` lifetime, Dioxus will automatically compare the props before
//! diffing the component. If the props don't change (according to PartialEq), the component will not be re-rendered.
//!
//! However, if the props use some generics or borrow from their parent, then Dioxus can't safely supress updates,
//! and is forced to render the child. If you think that this behavior is wrong for your usecase, you can implement
//! the memo method yourself, but beware, doing so is UNSAFE and may cause issues if you do it wrong.
//!
//! If you want to gain that little bit extra performance, consider using global state management, signals, or
//! memoized collections like im-rc which are designed for this use case.

use dioxus::prelude::*;

// By default, components with no props are always memoized.
// A props of () is considered empty.
pub fn Example(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "100% memoized!" }
    })
}

// These props do not borrow any content, and therefore can be safely memoized.
// However, the parent *must* create a new string on every render.
// Notice how these props implement PartialEq - this is required for 'static props
#[derive(PartialEq, Props)]
pub struct MyProps1 {
    name: String,
}

pub fn Example1(cx: Scope<MyProps1>) -> Element {
    cx.render(rsx! {
        div { "100% memoized! {cx.props.name}" }
    })
}

// These props do not borrow any content, and therefore can be safely memoized.
// In contrast with the `String` example, these props use `Rc<str>` which operates similar to strings in JavaScript.
// These strings cannot be modified, but may be cheaply shared in many places without issue.
#[derive(PartialEq, Props)]
pub struct MyProps2 {
    name: std::rc::Rc<str>,
}

pub fn Example2(cx: Scope<MyProps2>) -> Element {
    cx.render(rsx! {
        div { "100% memoized! {cx.props.name}" }
    })
}

// These props *do* borrow any content, and therefore cannot be safely memoized!.
#[derive(PartialEq, Props)]
pub struct MyProps3<'a> {
    name: &'a str,
}
// We need to manually specify a lifetime that ensures props and scope (the component's state) share the same lifetime.
// Using the `pub fn Example(cx: Scope): Component` pattern _will_ specify a lifetime, but that lifetime will be static which might
// not exactly be what you want
fn Example3(cx: Scope<'a, MyProps3<'a>>) -> DomTree {
    cx.render(rsx! {
        div { "Not memoized! {cx.props.name}" }
    })
}

// These props *do* borrow any content, and therefore cannot be safely memoized!.
// However, they cannot be compared, so we don't need the PartialEq flag.
#[derive(Props)]
pub struct MyProps4<'a> {
    onhandle: &'a dyn Fn(),
}

// We need to manually specify a lifetime that ensures props and scope (the component's state) share the same lifetime.
fn Example4<'a>(cx: Scope<'a, MyProps4<'a>>) -> DomTree {
    cx.render(rsx! {
        div { "Not memoized!", onclick: move |_| (props.onhandle)() }
    })
}
