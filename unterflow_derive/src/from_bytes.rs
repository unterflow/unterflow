use quote::Tokens;
use syn::{DeriveInput, Field, Variant, VariantData, Body, ConstExpr, Lit};

pub fn expand(ast: &DeriveInput) -> Tokens {
    match ast.body {
        Body::Struct(VariantData::Struct(ref body)) => expand_struct(ast, body),
        Body::Enum(ref variants) => expand_enum(ast, variants),
        _ => panic!("#[derive(FromBytes)] can only be used with structs or enums"),
    }
}

fn expand_struct(ast: &DeriveInput, body: &Vec<Field>) -> Tokens {
    let name = &ast.ident;

    let fields: Vec<_> = body.iter()
        .filter_map(|field| field.ident.as_ref())
        .map(|ident| quote! { #ident: FromBytes::from_bytes(reader)? })
        .collect();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics FromBytes for #name #ty_generics #where_clause {
            // allow empty implementations, i.e. SingleMessageHeader
            #[allow(unused_variables)]
            fn from_bytes(reader: &mut ::std::io::Read) -> Result<Self> {
                Ok(#name { #(#fields),* })
            }
        }
    }
}

fn expand_enum(ast: &DeriveInput, variants: &Vec<Variant>) -> Tokens {
    let name = &ast.ident;

    let variants: Vec<_> = variants.iter()
        .filter(|variant| variant.ident != "Unknown")
        .enumerate()
        .map(|(idx, variant)| {
            let value = match variant.discriminant {
                Some(ConstExpr::Lit(Lit::Int(value, _))) => value as u8,
                _ => idx as u8
            };

            let unqualified_ident = &variant.ident;
            let ident = quote! { #name::#unqualified_ident };
            quote! { #value => #ident } 
        })
        .collect();

    quote! {
        impl FromBytes for #name {
            fn from_bytes(reader: &mut ::std::io::Read) -> Result<Self> {
                let value = u8::from_bytes(reader)?;

                let value = match value {
                #(#variants),*,
                _ => #name::Unknown
                };

                Ok(value)
            }
        }
    }
}