use byteorder::{LittleEndian, ReadBytesExt};
use convert::*;
use errors::*;
use std::io::{Read, Seek, SeekFrom, Cursor};
use protocol::client::BrokerEventMetadata;

const BLOCK_SIZE: u16 = 4096;
const CACHE_LINE_LENGTH: u16 = 64;
const CACHE_LINE_PADDING: u16 = 2 * CACHE_LINE_LENGTH - 4;

#[derive(Debug, PartialEq)]
pub struct FsLogSegment {
    pub id: u32,
    pub version: u16,
    pub capacity: u32,
    pub size: u32,
}

 impl BlockLength for FsLogSegment {
     fn block_length() -> u16 {
         let offset = 8 + 4 * CACHE_LINE_LENGTH;
         align!(offset, BLOCK_SIZE)
     }
 }



impl FromBytes for FsLogSegment {

    fn from_bytes<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let id = FromBytes::from_bytes(reader)?;
        let version = FromBytes::from_bytes(reader)?;

        reader.seek(SeekFrom::Current(2))?; // skip unused
        let capacity = FromBytes::from_bytes(reader)?;

        reader.seek(SeekFrom::Current(CACHE_LINE_PADDING as i64))?; // skip cache line padding
        let size = FromBytes::from_bytes(reader)?;;

        let read_bytes = 16 + CACHE_LINE_PADDING;
        let skip = (FsLogSegment::block_length() - read_bytes) as i64;

        reader.seek(SeekFrom::Current(skip))?; // skip remaining empty space

        Ok(FsLogSegment {
            id,
            version,
            capacity,
            size
        })
    }

}

#[derive(Debug, PartialEq, Default)]
pub struct LogEntry {
    pub version: u16,
    pub position: u64,
    pub producer_id: u32,
    pub source_event_stream_partition_id: u32,
    pub source_event_position: u64,
    pub key: u64,
    pub topic_name: String,
    pub metadata: BrokerEventMetadata,
    pub value: Data,
}

impl FromBytes for LogEntry {

    fn from_bytes<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let version = FromBytes::from_bytes(reader)?;
        reader.seek(SeekFrom::Current(2))?; // skip reserved
        let position = FromBytes::from_bytes(reader)?;
        let producer_id = FromBytes::from_bytes(reader)?;
        let source_event_stream_partition_id = FromBytes::from_bytes(reader)?;
        let source_event_position = FromBytes::from_bytes(reader)?;
        let key = FromBytes::from_bytes(reader)?;

        let topic_name_length = reader.read_u16::<LittleEndian>()?;
        let metadata_length = reader.read_u16::<LittleEndian>()?;

        let topic_name = {
            let mut buffer = Vec::with_capacity(topic_name_length as usize);
            let mut handle = reader.take(topic_name_length as u64);
            handle.read_to_end(&mut buffer)?;
            String::from_utf8(buffer)?
        };

        let metadata = {
            let mut buffer = Vec::with_capacity(topic_name_length as usize);
            let mut handle = reader.take(metadata_length as u64);
            handle.read_to_end(&mut buffer)?;
            let mut cursor = Cursor::new(buffer);
            BrokerEventMetadata::from_bytes(&mut cursor)?
        };

        let value = {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;
            Data(buffer)
        };

        Ok(LogEntry {
            version,
            position,
            producer_id,
            source_event_stream_partition_id,
            source_event_position,
            key,
            topic_name,
            metadata,
            value
        })
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;
    use protocol::transport::{FrameHeader, FrameType};

    macro_rules! cursor {
        ($reader:ident, $file:expr) => (
            let data = include_bytes!(concat!("../../../data/", $file)).to_vec();
            let mut $reader = Cursor::new(data);
        )
    }

    #[test]
    fn test_decode_fs_log_segment() {
        cursor!(reader, "create-task/logs/default-topic.0/00.data");

        let segment = FsLogSegment::from_bytes(&mut reader).unwrap();
        assert_eq!(segment.id, 0);
        assert_eq!(segment.version, 0);
        assert_eq!(segment.capacity, 536870912);
        assert_eq!(segment.size, 4864);

        assert_eq!(reader.position(), 4096);

        let frame = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(frame.length, 92);
        assert_eq!(frame.version, 0);
        assert_eq!(frame.flags, 0);
        assert_eq!(frame.type_id, FrameType::Message);
        assert_eq!(frame.stream_id, 0);

        let entry = LogEntry::from_bytes(&mut reader).unwrap();
        assert_eq!(entry.version, 0);
        assert_eq!(entry.position, 4294967296);
        assert_eq!(entry.producer_id, u32::max_value());
        assert_eq!(entry.source_event_stream_partition_id, u32::max_value());
        assert_eq!(entry.source_event_position, u64::max_value());
        assert_eq!(entry.key, 4294967296);
        assert_eq!(entry.topic_name, String::new());
        // not sure about that
        assert_eq!(entry.metadata.req_channel_id, 13107243);
        assert_eq!(entry.value, Data(Vec::new()));
    }
}
