#![allow(non_snake_case)]

pub(crate) mod arbitrary_value;
pub(crate) mod diff;
pub(crate) mod events;
pub(crate) mod lazynodes;
pub(crate) mod mutations;
pub(crate) mod nodes;
pub(crate) mod properties;
pub(crate) mod scopes;
pub(crate) mod util;
pub(crate) mod virtual_dom;

pub(crate) mod innerlude {
    pub use crate::arbitrary_value::*;
    pub use crate::events::*;
    pub use crate::lazynodes::*;
    pub use crate::mutations::*;
    pub use crate::nodes::*;
    pub use crate::properties::*;
    pub use crate::scopes::*;
    pub use crate::util::*;
    pub use crate::virtual_dom::*;

    /// An [`Element`] is a possibly-none [`VNode`] created by calling `render` on [`Scope`] or [`ScopeState`].
    ///
    /// Any [`None`] [`Element`] will automatically be coerced into a placeholder [`VNode`] with the [`VNode::Placeholder`] variant.
    pub type Element<'a> = Option<VNode<'a>>;

    /// A [`Component`] is a function that takes a [`Scope`] and returns an [`Element`].
    ///
    /// Components can be used in other components with two syntax options:
    /// - lowercase as a function call with named arguments (rust style)
    /// - uppercase as an element (react style)
    ///
    /// ## Rust-Style
    ///
    /// ```rust, ignore
    /// fn example(cx: Scope<Props>) -> Element {
    ///     // ...
    /// }
    ///
    /// rsx!(
    ///     example()
    /// )
    /// ```
    /// ## React-Style
    /// ```rust, ignore
    /// fn Example(cx: Scope<Props>) -> Element {
    ///     // ...
    /// }
    ///
    /// rsx!(
    ///     Example {}
    /// )
    /// ```
    ///
    /// ## As a closure
    /// This particular type alias lets you even use static closures for pure/static components:
    ///
    /// ```rust, ignore
    /// static Example: Component<Props> = |cx| {
    ///     // ...
    /// };
    /// ```
    pub type Component<P = ()> = fn(Scope<P>) -> Element;
}

pub use crate::innerlude::{
    AnyEvent, Attribute, Component, DioxusElement, ElementId, EventPriority, Listener, NodeFactory,
    ScopeId, UiEvent, UserEvent, VNode,
};

/// The purpose of this module is to alleviate imports of many common types
///
/// This includes types like [`Scope`], [`Element`], and [`Component`].
pub mod prelude {
    pub use crate::innerlude::{Element, LazyNodes, NodeFactory, Scope, VNode};
}

pub mod exports {
    //! Important dependencies that are used by the rest of the library
    //! Feel free to just add the dependencies in your own Crates.toml
    pub use bumpalo;
}

/// Functions that wrap unsafe functionality to prevent us from misusing it at the callsite
pub(crate) mod unsafe_utils {
    use crate::VNode;

    pub(crate) unsafe fn extend_vnode<'a, 'b>(node: &'a VNode<'a>) -> &'b VNode<'b> {
        std::mem::transmute(node)
    }
}
