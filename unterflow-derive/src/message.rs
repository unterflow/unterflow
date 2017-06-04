use quote::Tokens;
use syn::{DeriveInput, Body};
use util;

pub fn expand(ast: &DeriveInput) -> Tokens {
    match ast.body {
        Body::Struct(_) => expand_struct(ast),
        _ => panic!("#[derive(Message)] can only be used with structs"),
    }
}

fn expand_struct(ast: &DeriveInput) -> Tokens {
    let name = &ast.ident;

    let template_id = util::template_id(ast).expect("#[derive(Message)] requires message(template_id) attribute]");
    let schema_id = util::schema_id(ast).expect("#[derive(Message)] requires message(schema_id) attribute");
    let version = util::version(ast).expect("#[derive(Message)] requires message(version) attribute]");

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics Message for #name #ty_generics #where_clause {
            fn template_id() -> u16 {
                #template_id
            }

            fn schema_id() -> u16 {
                #schema_id
            }

            fn version() -> u16 {
                #version
            }

        }
    }
}