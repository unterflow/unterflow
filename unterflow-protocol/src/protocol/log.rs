use convert::*;
use errors::*;
use std::io::{Read, Seek};

const BLOCK_SIZE: u16 = 4096;
const CACHE_LINE_LENGTH: u16 = 64;

#[derive(Debug, PartialEq)]
pub struct FsLogSegment {
    pub id: u32,
    pub version: u16,
    pub capacity: u32,
    pub size: u32,
}

impl BlockLength for FsLogSegment {
    fn block_length() -> u16 {
        // see FsLogSegmentDescriptor.java
        let offset = 8 + 4 * CACHE_LINE_LENGTH;

        align!(offset, BLOCK_SIZE)
    }
}

impl FromBytes for FsLogSegment {

    fn from_bytes<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let id = FromBytes::from_bytes(reader)?;
        let version = FromBytes::from_bytes(reader)?;
        let capacity = FromBytes::from_bytes(reader)?;
        let size = FromBytes::from_bytes(reader)?;;

        Ok(FsLogSegment {
            id,
            version,
            capacity,
            size
        })
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    macro_rules! cursor {
        ($reader:ident, $file:expr) => (
            let data = include_bytes!(concat!("../../../data/", $file)).to_vec();
            let mut $reader = Cursor::new(data);
        )
    }

    #[test]
    fn test_block_length() {
        assert_eq!(FsLogSegment::block_length(), 4096);
    }

    #[test]
    fn test_decode_fs_log_segment() {
        cursor!(reader, "create-task/logs/default-topic.0/00.data");

        let segment = FsLogSegment::from_bytes(&mut reader).unwrap();
        assert_eq!(segment.id, 0);
        assert_eq!(segment.version, 0);
        assert_eq!(segment.capacity, 536870912);
        assert_eq!(segment.size, 4864);
    }
}
