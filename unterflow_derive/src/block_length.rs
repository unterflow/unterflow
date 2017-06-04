use quote::Tokens;
use syn::{DeriveInput, Field, VariantData, Body, Ty};
use util::{enum_type, as_ty};

pub fn expand(ast: &DeriveInput) -> Tokens {
    let name = &ast.ident;

    let block_length = match ast.body {
        Body::Struct(VariantData::Struct(ref body)) => expand_struct(body),
        Body::Enum(_) => expand_enum(ast),
        _ => panic!("#[derive(BlockLength)] can only be used with structs or enums"),
    };

    quote! {
        impl BlockLength for #name {
            fn block_length() -> u16 {
                #block_length as u16
            }
        }
    }

}

fn expand_struct(body: &Vec<Field>) -> Tokens {
    let mut fields: Vec<_> = body.iter()
        .filter(|field| match field.ty {
            // exclude Vec and Strings from block length
            Ty::Path(None, ref path) => !path.segments.iter().any(|seg| seg.ident == "Vec" || seg.ident == "String"),
            _ => false
        }) 
        .map(|field| {
            let ref ty = field.ty;
            quote! { #ty::block_length() }
        })
        .collect();

    // allow empty implementations, i.e. SingleMessageHeader
    fields.push(quote! { 0 } );

    quote! {
        #(#fields)+*
    }
}

fn expand_enum(ast: &DeriveInput) -> Tokens {
    let ty = enum_type(ast)
        .unwrap_or(as_ty("u8".to_string()));

    quote! {
        ::std::mem::size_of::<#ty>()
    }
}