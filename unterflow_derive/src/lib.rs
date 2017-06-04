extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod from_bytes;
mod enum_default;
mod block_length;

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = from_bytes::expand(&ast);
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumDefault)]
pub fn derive_enum_default(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = enum_default::expand(&ast);
    expanded.parse().unwrap()
}

#[proc_macro_derive(BlockLength, attributes(block_length))]
pub fn derive_block_length(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = block_length::expand(&ast);
    expanded.parse().unwrap()
}