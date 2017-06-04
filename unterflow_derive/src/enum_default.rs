use quote::Tokens;
use syn::{DeriveInput, Body};

pub fn expand(ast: &DeriveInput) -> Tokens {
    match ast.body {
        Body::Enum(_) => expand_enum(ast),
        _ => panic!("#[derive(EnumDefault)] can only be used with enums"),
    }
}

fn expand_enum(ast: &DeriveInput) -> Tokens {
    let name = &ast.ident;

    quote! {
        impl Default for #name {
            fn default() -> Self {
                #name::Unknown
            }
        }
    }
}