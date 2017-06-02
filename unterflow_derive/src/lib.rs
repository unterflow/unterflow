extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = expand_derive_from_bytes(&ast);
    expanded.parse().unwrap()
}

fn expand_derive_from_bytes(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let from_bytes = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref body)) => {
            let fields: Vec<_> = body.iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { #ident: FromBytes::from_bytes(reader)? })
                .collect();

            quote! { #name { #(#fields),* } }
        },
        _ => panic!("#[derive(FromBytes)] can only be used with structs"),
    };

    // Used in the quasi-quotation below as `#name`

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        // The generated impl
        impl #impl_generics FromBytes for #name #ty_generics #where_clause {
            fn from_bytes(reader: &mut Read) -> Result<Self> {
                Ok(#from_bytes)
            }
        }
    }
}