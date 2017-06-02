#[macro_use]
extern crate error_chain;

extern crate byteorder;

#[macro_use]
extern crate unterflow_macro;

#[macro_use]
extern crate unterflow_derive;


mod errors;

use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

use errors::*;

pub trait FromBytes: Sized {

    fn from_bytes(reader: &mut Read) -> Result<Self>;

}

impl FromBytes for u8 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u8()?)
    }
}

impl FromBytes for u16 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u16::<LittleEndian>()?)
    }
}

impl FromBytes for u32 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}


impl FromBytes for u64 {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }
}

impl FromBytes for String {
    fn from_bytes(reader: &mut Read) -> Result<Self> {
        let length = reader.read_u16::<LittleEndian>()? as usize;
        let mut buffer = Vec::with_capacity(length);
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
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

message!(Foo {
    a: u16,
    b: u32,
    c: u64,
    d: Vec<u8>
});

#[derive(FromBytes, Debug, PartialEq)]
struct Bar {
    a: u16,
    b: u32,
    c: u64,
    d: Vec<u8>,
}


#[cfg(test)]
mod tests {

    use super::*;
    
    use std::io::Cursor;

    #[test]
    fn test_macro() {
        let mut reader = Cursor::new(vec![12, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 4, 0, 1, 2, 3, 4]);
        let foo = Foo::from_bytes(&mut reader).unwrap();

        assert_eq!(foo.a, 12);
        assert_eq!(foo.b, 13);
        assert_eq!(foo.c, 14);
        assert_eq!(foo.d, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_derive() {
        let mut reader = Cursor::new(vec![12, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 4, 0, 1, 2, 3, 4]);
        let bar = Bar::from_bytes(&mut reader).unwrap();

        assert_eq!(bar.a, 12);
        assert_eq!(bar.b, 13);
        assert_eq!(bar.c, 14);
        assert_eq!(bar.d, vec![1, 2, 3, 4]);
    }

}
