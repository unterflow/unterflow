use quote::Tokens;
use syn::{DeriveInput, Field, VariantData, Body, MetaItem, Lit, Ty};

pub fn expand(ast: &DeriveInput) -> Tokens {
    let name = &ast.ident;

    let block_length = ast.attrs.iter()
        .filter_map(|attr|
            match attr.value {
                MetaItem::NameValue(ref ident, Lit::Str(ref value, _)) if ident == "block_length" => value.parse::<usize>().ok(),
                _ => None
            }
        )
        .map(|value| quote! { #value })
        .next()
        .unwrap_or(
            match ast.body {
                Body::Struct(VariantData::Struct(ref body)) => expand_struct(body),
                Body::Enum(_) => expand_enum(),
                _ => panic!("#[derive(BlockLength)] can only be used with structs or enums or has to be specified with #[block_length] attribute"),
            }
        );

    quote! {
        impl BlockLength for #name {
            fn block_length() -> usize {
                #block_length
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

fn expand_enum() -> Tokens {
    quote! {
        ::std::mem::size_of::<u8>()
    }
}