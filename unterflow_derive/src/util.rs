use syn::{DeriveInput, MetaItem, Lit, Ty, Ident, Path, NestedMetaItem};

pub fn named_attr(ast: &DeriveInput, name: &str) -> Option<String> {
    ast.attrs.iter()
        .filter_map(|attr|
            match attr.value {
                MetaItem::NameValue(ref ident, Lit::Str(ref value, _)) if ident == name => Some(value.to_owned()),
                _ => None
            }
        )
        .next()
}

pub fn list_attr(ast: &DeriveInput, name: &str, item: &str) -> Option<String> {
    ast.attrs.iter()
        .filter_map(|attr|
            match attr.value {
                MetaItem::List(ref ident, ref values) if ident == name => {
                    values.iter()
                        .filter_map(|attr|
                            match *attr {
                                NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, Lit::Str(ref value, _))) if ident == item => Some(value.to_owned()),
                                _ => None
                            }
                         )
                         .next()
                }
                _ => None
            }
        )
        .next()
}

pub fn enum_type(ast: &DeriveInput) -> Option<Ty> {
    named_attr(ast, "enum_type").map(|value| as_ty(value))
}

pub fn template_id(ast: &DeriveInput) -> Option<u16> {
    list_attr(ast, "message", "template_id").and_then(|value| value.parse::<u16>().ok())
}

pub fn schema_id(ast: &DeriveInput) -> Option<u16> {
    list_attr(ast, "message", "schema_id").and_then(|value| value.parse::<u16>().ok())
}
pub fn version(ast: &DeriveInput) -> Option<u16> {
    list_attr(ast, "message", "version").and_then(|value| value.parse::<u16>().ok())
}

pub fn as_ty(ty: String) -> Ty {
    let ident = Ident::from(ty);
    Ty::Path(None, Path::from(ident))
}