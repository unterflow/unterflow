use syn::{DeriveInput, MetaItem, Lit, Ty, Ident, Path};

pub fn enum_type(ast: &DeriveInput) -> Option<Ty> {
    ast.attrs.iter()
        .filter_map(|attr|
            match attr.value {
                MetaItem::NameValue(ref ident, Lit::Str(ref value, _)) if ident == "enum_type" => Some(as_ty(value)),
                _ => None
            }
        )
        .next()
}


pub fn as_ty(ty: &str) -> Ty {
    let ident = Ident::from(ty.to_owned());
    Ty::Path(None, Path::from(ident))
}