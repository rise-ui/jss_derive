#![recursion_limit = "256"]

extern crate proc_macro2;
extern crate proc_macro;
extern crate inflector;
extern crate regex;
extern crate syn;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_extract;
#[macro_use]
extern crate quote;

mod common;
mod parser;
mod setters;

use syn::DeriveInput;

#[proc_macro_derive(ImplPropertySetters)]
pub fn setters_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    if let syn::Data::Struct(data_struct) = ast.data {
        setters::get_impl_trait_tokens(ast.ident, data_struct).into()
    } else {
        panic!("Only `struct`s could be derived using the macro")
    }
}

#[proc_macro_derive(ImplStyleParser)]
pub fn parser_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    if let syn::Data::Struct(data_struct) = ast.data {
        parser::get_impl_trait_tokens(ast.ident, data_struct).into()
    } else {
        panic!("Only `struct`s could be derived using the macro")
    }
}
