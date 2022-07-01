# Describing the UI

Dioxus is a *declarative* framework. This means that instead of telling Dioxus what to do (e.g. to "create an element" or "set color to red") we simply *declare* what we want the UI to look like using RSX.

You have already seen a simple example or RSX syntax in the "hello world" application:

```rust
{{#include ../../examples/hello_world_desktop.rs}}
```

Here, we use the `rsx!` macro to *declare* that we want a `div` element, containing the text `"Hello, world!"`. Dioxus takes the RSX and constructs a UI from it.

## RSX Features

RSX is very similar to HTML in that it describes elements with attributes and children. Here's an empty `div` element in RSX, as well as the resulting HTML:

```rust
{{#include ../../examples/rsx_overview.rs:empty}}
```
```html
<div></div>
```

> Note: Though the examples here use Dioxus Desktop, and render HTML elements, the RSX sytax is the same for all renderers. The only difference might be that, instead of outputing HTML elements, a TUI renderer, for example, might output nodes to be rendered to the terminal.

### Children

To add children to an element, put them inside the `{}` brackets. They can be either other elements, or text. For example, you could have an `ol` (ordered list) element, containing 3 `li` (list item) elements, each of which contains some text: 

```rust
{{#include ../../examples/rsx_overview.rs:children}}
```
```html
<ol>
    <li>First Item</li>
    <li>Second Item</li>
    <li>Third Item</li>
</ol>
```

### Attributes

Attributes are also specified inside the `{}` brackets, using the `name: value` syntax. You can provide the value as a literal in the RSX:
```rust
{{#include ../../examples/rsx_overview.rs:attributes}}
```
```html
<a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" class="primary_button" autofocus="true">Log In</a>
```

You can also provide variables to set attributes:
```rust
{{#include ../../examples/rsx_overview.rs:variable_attributes}}
```
```html

<button disabled="true" class="button">Rewrite it in rust</button>
```

> Note: All attributes defined in `dioxus-html` follow the snake_case naming convention. They transform their `snake_case` names to HTML's `camelCase` attributes.

#### Custom Attributes

Dioxus has a pre-configured set of attributes that you can use. RSX is validated at compile time to make sure you didn't specify an invalid attribute. If you want to override this behavior with a custom attribute name, specify the attribute in quotes:

```rust
{{#include ../../examples/rsx_overview.rs:custom_attributes}}
```
```html
<b customAttribute="value">
    Rust is cool
</b>
```

### Interpolation

Similarly to how you can [format](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html) Rust strings, you can also interpolate in RSX text. Use `{variable}` to Display the value of a variable in a string, or `{variable:?}` to use the Debug representation:

```rust
{{#include ../../examples/rsx_overview.rs:formatting}}
```
```html
<button class="country-es">Coordinates: (42, 0)</button>
```

### Expressions

You can include arbitrary Rust expressions within RSX, but you must escape them in `[]` brackets:

```rust
{{#include ../../examples/rsx_overview.rs:expression}}
```
```html
<span>DIOXUS</span>
```