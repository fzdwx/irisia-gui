use expr::state_block::{parse_stmts, stmts_to_tokens};
use proc_macro::TokenStream;
use quote::quote;
use style::StyleCodegen;
use syn::{parse::Parser, parse_macro_input, DeriveInput, ItemFn};

mod derive_style;
mod derive_style_reader;
mod element;
pub(crate) mod expr;
mod main_macro;
mod style;

/// To build a element tree visually. This macro will returns
/// a type implements `StructureBuilder`. Call `into_rendering`
/// on it to let it goes into rendering mode, which allows you
/// rendering them on canvas.
///
/// # Syntax
/// ### Example
/// ```no_run
/// cream::build! {
///     RootElement {
///         prop1: "hello world",
///         prop2: &self.some_field,
///         +style: cream::style!{
///             ...
///         },
///         +listen: "you'll receive this str as key in your `event_dispatcher.recv()`",
///         
///         Element1;
///         
///         if 1 + 1 == 2 {
///             match Some("this is some") {
///                 Some(s) => Element2 {
///                     display: s
///                 },
///                 None => {}
///             }
///         } else {
///             Element3;
///         }
///         
///         for _ in 0..3 {
///             // @key _; // `key` command is optional
///             Element3;
///         }
///
///         while let Some(value_i32) = some_iter.next() {
///             @key value_i32;
///             Element4;
///         }
///
///         @extend element_tree;
///     }
/// }
/// ```
///
/// # Example
/// ```no_run
/// let element_tree = cream::build! {
///     ...
/// };
///
/// element_tree.into_rendering().finish(drawing_region)?;
/// ```
#[proc_macro]
pub fn build(input: TokenStream) -> TokenStream {
    match element::build::build.parse(input) {
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    let stmts = match parse_stmts::<StyleCodegen>.parse(input) {
        Ok(stmts) => stmts,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut stmt_tokens = proc_macro2::TokenStream::new();
    stmts_to_tokens(&mut stmt_tokens, &stmts);

    quote! {{
        use cream::style::StyleContainer;
        #stmt_tokens
    }}
    .into()
}

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);
    match main_macro::main_macro(item_fn) {
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input as DeriveInput);

    let (impl_gen, type_gen, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_gen cream::Event for #ident #type_gen
        #where_clause
        {}
    }
    .into()
}

#[proc_macro_derive(Style, attributes(cream))]
pub fn derive_style(input: TokenStream) -> TokenStream {
    match derive_style::derive_style(parse_macro_input!(input as DeriveInput)) {
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(StyleReader)]
pub fn derive_style_reader(input: TokenStream) -> TokenStream {
    match derive_style_reader::derive_style_reader(parse_macro_input!(input as DeriveInput)) {
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    }
}