use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;

mod ifmt;
mod inlineprops;
mod props;
mod rsx;

#[proc_macro]
pub fn format_args_f(input: TokenStream) -> TokenStream {
    use ifmt::*;
    let item = parse_macro_input!(input as IfmtInput);
    format_args_f_impl(item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Props, attributes(props))]
pub fn derive_typed_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match props::impl_my_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn rsx(s: TokenStream) -> TokenStream {
    // let s: TokenStream = match syn::parse::<rsx::CallBody>(s) {
    //     Err(err) => err.to_compile_error().into(),
    //     Ok(stream) => stream.to_token_stream().into(),
    // };

    // panic!("{:}", s.to_string());
    // return s;

    let stream = quote! {
        LazyNodes ::
           new(move | __cx : NodeFactory | -> VNode
           {
               use dioxus_elements :: { GlobalAttributes, SvgAttributes } ;
               __cx.element(dioxus_elements :: input, __cx.bump().alloc([]),
               __cx.bump().alloc([dioxus_elements ::
               input.r#type(__cx, format_args_f! ("text"))]), __cx.bump().alloc([]),
               None,)
           })
    };

    stream.into()
}

#[proc_macro_attribute]
pub fn inline_props(_args: proc_macro::TokenStream, s: TokenStream) -> TokenStream {
    match syn::parse::<inlineprops::InlinePropsBody>(s) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}
