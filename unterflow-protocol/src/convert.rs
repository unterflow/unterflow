use std::fmt;
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use rmpv::decode::read_value;
use rmpv::{Value, Utf8String};
use protocol::transport::MessageHeader;
use errors::*;

pub trait FromBytes: Sized {
    fn from_bytes(reader: &mut Read) -> Result<Self>;
}

pub trait BlockLength {
    fn block_length() -> u16;
    fn skip_block(buffer: &mut Seek) -> Result<u64> {
        let length = Self::block_length();
        Ok(buffer.seek(SeekFrom::Current(length as i64))?)
    }
}

pub trait Message {
    fn template_id() -> u16;
    fn schema_id() -> u16;
    fn version() -> u16;
}

pub trait ToMessageHeader {
    fn message_header() -> MessageHeader;
}

#[derive(PartialEq, Default)]
pub struct Data(Vec<u8>);

impl ::std::ops::Deref for Data {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl From<Data> for Vec<u8> {
    fn from(data: Data) -> Self {
        data.0
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut reader = Cursor::new(&self.0);
        if let Ok(value) = read_value(&mut reader) {
            // assume all message pack data has to be a map
            // to distinguish non message pack data which still
            // can be parsed somehow
            if value.is_map() {
                write!(f, "{}", value)?;

                if let Some(values) = value.as_map() {
                    let payload_key = Value::String(Utf8String::from("payload"));
                    let payload = values.iter().find(|&&(ref key, _)| key == &payload_key);

                    if let Some(&(_, Value::Binary(ref bytes))) = payload {
                        let mut reader = Cursor::new(bytes);
                        if let Ok(value) = read_value(&mut reader) {
                            write!(f, ", payload (decoded): {}", value)?;
                        }
                    }
                }

                return Ok(());
            }
        }

        // default debug output
        write!(f, "Data({:?})", self.0)
    }
}

impl FromBytes for u8 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u8()?)
    }
}

impl FromBytes for i8 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_i8()?)
    }
}

impl FromBytes for u16 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u16::<LittleEndian>()?)
    }
}

impl FromBytes for i16 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_i16::<LittleEndian>()?)
    }
}

impl FromBytes for u32 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}

impl FromBytes for i32 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_i32::<LittleEndian>()?)
    }
}

impl FromBytes for u64 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }
}

impl FromBytes for i64 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_i64::<LittleEndian>()?)
    }
}

impl FromBytes for Data {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        let length = reader.read_u16::<LittleEndian>()?;
        let mut buffer = Vec::with_capacity(length as usize);
        let mut handle = reader.take(length as u64);
        handle.read_to_end(&mut buffer)?;
        Ok(Data(buffer))
    }
}

impl FromBytes for String {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        let buffer: Data = FromBytes::from_bytes(reader)?;
        Ok(String::from_utf8(buffer.to_vec())?)
    }
}

impl<T: FromBytes> FromBytes for Vec<T> {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        let _block_length = reader.read_u16::<LittleEndian>()?;
        let num_in_group = reader.read_u8()?;
        let mut group: Vec<T> = Vec::with_capacity(num_in_group as usize);
        for _ in 0..num_in_group {
            group.push(T::from_bytes(reader)?);
        }
        Ok(group)
    }
}

impl FromBytes for bool {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        match reader.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            x => bail!("Invalid boolean encoded as {:?}", x),
        }
    }
}


macro_rules! impl_block_length {
    ($t:ty) => (
        impl BlockLength for $t {
            fn block_length() -> u16 {
                ::std::mem::size_of::<$t>() as u16
            }
        }
    )
}

impl_block_length!(u8);
impl_block_length!(i8);
impl_block_length!(u16);
impl_block_length!(i16);
impl_block_length!(u32);
impl_block_length!(i32);
impl_block_length!(u64);
impl_block_length!(i64);
impl_block_length!(bool);


#[cfg(test)]
#[allow(unused)]
mod tests {

    use super::*;

    #[test]
    fn test_derive_from_bytes_struct() {
        #[derive(FromBytes)]
        struct Foo {
            a: u8,
            b: u16,
            c: u32,
            d: bool,
            e: Data,
            f: String,
        }

        let mut bytes: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 1, 3, 0, 8, 9, 10, 4, 0, 97, 98, 99, 100];
        let foo = Foo::from_bytes(&mut bytes).unwrap();

        assert_eq!(foo.a, 1);
        assert_eq!(foo.b, 770);
        assert_eq!(foo.c, 117835012);
        assert_eq!(foo.d, true);
        assert_eq!(foo.e, Data(vec![8, 9, 10]));
        assert_eq!(foo.f, "abcd");
    }

    #[test]
    fn test_derive_from_bytes_enum() {
        #[derive(Debug, PartialEq, FromBytes)]
        enum Foo {
            A,
            B,
            C = 100,
            D = 122,
            Unknown,
        }

        let mut bytes: &[u8] = &[0, 1, 100, 122, 233];
        let mut next = || Foo::from_bytes(&mut bytes).unwrap();

        assert_eq!(next(), Foo::A);
        assert_eq!(next(), Foo::B);
        assert_eq!(next(), Foo::C);
        assert_eq!(next(), Foo::D);
        assert_eq!(next(), Foo::Unknown);
    }

    #[test]
    fn test_derive_from_bytes_enum_type() {
        #[derive(Debug, PartialEq, FromBytes)]
        #[enum_type = "u32"]
        enum Foo {
            A,
            B = 16909060,
            Unknown,
        }

        let mut bytes: &[u8] = &[0, 0, 0, 0, 4, 3, 2, 1, 1, 0, 0, 0];
        let mut next = || Foo::from_bytes(&mut bytes).unwrap();

        assert_eq!(next(), Foo::A);
        assert_eq!(next(), Foo::B);
        assert_eq!(next(), Foo::Unknown);
    }

    #[test]
    fn test_derive_from_bytes_nested() {
        #[derive(Debug, PartialEq, FromBytes)]
        struct Foo {
            foo: u16,
            bar: Bar,
            baz: Baz,
        }

        #[derive(Debug, PartialEq, FromBytes)]
        enum Bar {
            A,
            B,
            Unknown,
        }

        #[derive(Debug, PartialEq, FromBytes)]
        struct Baz {
            baz: u16,
            bar: Bar,
        }


        let mut bytes: &[u8] = &[12, 0, 1, 0, 2, 0];
        let foo = Foo::from_bytes(&mut bytes).unwrap();

        assert_eq!(foo,
                   Foo {
                       foo: 12,
                       bar: Bar::B,
                       baz: Baz {
                           baz: 512,
                           bar: Bar::A,
                       },
                   });
    }

    #[test]
    fn test_derive_from_bytes_group_encoding() {
        #[derive(Debug, PartialEq, FromBytes)]
        struct Foo {
            foo: u16,
            bar: Vec<Bar>,
            baz: Data,
        }

        #[derive(Debug, PartialEq, FromBytes)]
        struct Bar {
            bar: u16,
            baz: String,
        }

        let mut bytes: &[u8] = &[12, 0, 2, 0, 3, 1, 0, 3, 0, 102, 111, 111, 2, 0, 3, 0, 98, 97, 114, 3, 0, 3, 0, 98, 97, 122, 3, 0, 1, 2,
                                 3];
        let foo = Foo::from_bytes(&mut bytes).unwrap();

        assert_eq!(foo,
                   Foo {
                       foo: 12,
                       bar: vec![Bar {
                                     bar: 1,
                                     baz: "foo".to_string(),
                                 },
                                 Bar {
                                     bar: 2,
                                     baz: "bar".to_string(),
                                 },
                                 Bar {
                                     bar: 3,
                                     baz: "baz".to_string(),
                                 }],
                       baz: Data(vec![1, 2, 3]),
                   });
    }

    #[test]
    fn test_derive_enum_default() {
        #[derive(Debug, PartialEq, EnumDefault)]
        enum Foo {
            A,
            B,
            Unknown,
        }

        assert_eq!(Foo::default(), Foo::Unknown);

        #[derive(Debug, PartialEq, Default)]
        struct Bar {
            a: u16,
            b: Foo,
        }

        assert_eq!(Bar::default(),
                   Bar {
                       a: 0,
                       b: Foo::Unknown,
                   });
    }

    #[test]
    fn test_derive_block_length_struct() {
        #[derive(BlockLength)]
        struct Foo {
            a: u8,
            b: u64,
            c: Vec<Bar>,
            d: Data,
            e: String,
        }

        struct Bar {
            a: u16,
        }

        assert_eq!(Foo::block_length(), 9);
    }

    #[test]
    fn test_derive_block_length_enum() {
        #[derive(BlockLength)]
        enum Foo {
            A,
            B,
            Unknown,
        }

        assert_eq!(Foo::block_length(), 1);
    }

    #[test]
    fn test_derive_block_length_enum_type() {
        #[derive(FromBytes, BlockLength)]
        #[enum_type = "u64"]
        enum Bar {
            A,
            B,
            Unknown,
        }


        assert_eq!(Bar::block_length(), 8);
    }


    #[test]
    fn test_derive_block_length_nested() {
        #[derive(Debug, PartialEq, BlockLength)]
        struct Foo {
            foo: u16,
            bar: Bar,
            baz: Baz,
        }

        #[derive(Debug, PartialEq, BlockLength)]
        #[enum_type = "u32"]
        enum Bar {
            A,
            B,
            Unknown,
        }

        #[derive(Debug, PartialEq, BlockLength)]
        struct Baz {
            baz: u16,
            bar: Bar,
        }

        assert_eq!(Foo::block_length(), 12);

    }

}
