use std::io::{Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use transport::MessageHeader;
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

impl FromBytes for Vec<u8> {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        let length = reader.read_u16::<LittleEndian>()?;
        let mut buffer = Vec::with_capacity(length as usize);
        let mut handle = reader.take(length as u64);
        handle.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl FromBytes for String {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        let buffer: Vec<u8> = FromBytes::from_bytes(reader)?;
        Ok(String::from_utf8(buffer)?)
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
            d: Vec<u8>,
            e: String,
        }

        let mut bytes: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 3, 0, 8, 9, 10, 4, 0, 97, 98, 99, 100];
        let foo = Foo::from_bytes(&mut bytes).unwrap();

        assert_eq!(foo.a, 1);
        assert_eq!(foo.b, 770);
        assert_eq!(foo.c, 117835012);
        assert_eq!(foo.d, vec![8, 9, 10]);
        assert_eq!(foo.e, "abcd");
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
            c: Vec<u8>,
            d: String,
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
