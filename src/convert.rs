use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use errors::*;

pub trait FromBytes: Sized {
    fn from_bytes(reader: &mut Read) -> Result<Self>;
}

pub trait BlockLength {
    fn block_length() -> usize;
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
        let length = reader.read_u16::<LittleEndian>()?;
        let mut buffer = Vec::with_capacity(length as usize);
        let mut handle = reader.take(length as u64);
        handle.read_to_end(&mut buffer)?;
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

macro_rules! block_length {
    ($t:ty) => (
        impl BlockLength for $t {
            fn block_length() -> usize {
                ::std::mem::size_of::<$t>()
            }
        }
    )
}

block_length!(u8);
block_length!(u16);
block_length!(u32);
block_length!(u64);
