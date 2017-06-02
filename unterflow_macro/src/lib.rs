#[macro_export]
macro_rules! message {
    ($name:ident{ $($i:ident: $t:ty),*}) => (
        #[derive(Debug, PartialEq)]
        struct $name {
            $($i: $t,)*
        }

        impl FromBytes for $name {

            fn from_bytes(reader: &mut Read) -> Result<Self> {
                Ok(Self {
                    $($i: FromBytes::from_bytes(reader)?,)*
                })
            }
        }
    )
}
