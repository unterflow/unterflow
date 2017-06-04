use convert::*;
use errors::*;

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength)]
pub struct FrameHeader {
    length: u32,
    version: u8,
    flags: u8,
    type_id: FrameType,
    stream_id: u32,
}

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
#[enum_type = "u16"]
pub enum FrameType {
    Message,
    Padding,
    ControlClose = 100,
    ControlEndOfStream = 101,
    ControlKeepAlive = 102,
    ProtocolControlFrame = 103,
    Unknown,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength)]
pub struct TransportHeader {
    protocol_id: TransportProtocol,
}

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
#[enum_type = "u16"]
enum TransportProtocol {
    RequestResponse,
    FullDuplexSingleMessage,
    Unknown,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength)]
pub struct RequestResponseHeader {
    connection_id: u64,
    request_id: u64,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength)]
pub struct SingleMessageHeader {}

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::Cursor;

    macro_rules! cursor {
        ($reader:ident, $file:expr) => (
            let data = include_bytes!($file).to_vec();
            let mut $reader = Cursor::new(data);
        )
    }

    #[test]
    fn test_block_length() {
        assert_eq!(FrameHeader::block_length(), 12);
        assert_eq!(FrameType::block_length(), 2);
        assert_eq!(TransportHeader::block_length(), 2);
        assert_eq!(RequestResponseHeader::block_length(), 16);
        assert_eq!(SingleMessageHeader::block_length(), 0);
    }

    #[test]
    fn test_decode_create_task_request() {
        cursor!(reader, "../dumps/create-task-request");

        let header = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.length, 147);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, 0);
        assert_eq!(header.type_id, FrameType::Message);
        assert_eq!(header.stream_id, 2);

        let header = TransportHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.protocol_id, TransportProtocol::RequestResponse);

        let header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.connection_id, 1);
        assert_eq!(header.request_id, 0);
    }

    #[test]
    fn test_decode_create_task_response() {
        cursor!(reader, "../dumps/create-task-response");

        let header = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.length, 271);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, 0);
        assert_eq!(header.type_id, FrameType::Message);
        assert_eq!(header.stream_id, 2);

        let header = TransportHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.protocol_id, TransportProtocol::RequestResponse);

        let header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.connection_id, 1);
        assert_eq!(header.request_id, 0);
    }

    #[test]
    fn test_decode_close_subscription_request() {
        cursor!(reader, "../dumps/close-subscription-request");

        let header = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.length, 90);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, 0);
        assert_eq!(header.type_id, FrameType::Message);
        assert_eq!(header.stream_id, 2);

        let header = TransportHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.protocol_id, TransportProtocol::RequestResponse);

        let header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.connection_id, 2);
        assert_eq!(header.request_id, 0);
    }

    #[test]
    fn test_decode_close_subscription_response() {
        cursor!(reader, "../dumps/close-subscription-response");

        let header = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.length, 89);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, 0);
        assert_eq!(header.type_id, FrameType::Message);
        assert_eq!(header.stream_id, 2);

        let header = TransportHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.protocol_id, TransportProtocol::RequestResponse);

        let header = RequestResponseHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.connection_id, 2);
        assert_eq!(header.request_id, 0);
    }

    #[test]
    fn test_decode_close_channel() {
        cursor!(reader, "../dumps/close-channel");

        let header = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.length, 0);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, 0);
        assert_eq!(header.type_id, FrameType::ControlClose);
        assert_eq!(header.stream_id, 0);
    }

    #[test]
    fn test_decode_end_of_stream() {
        cursor!(reader, "../dumps/end-of-stream");

        let header = FrameHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header.length, 0);
        assert_eq!(header.version, 0);
        assert_eq!(header.flags, 0);
        assert_eq!(header.type_id, FrameType::ControlEndOfStream);
        assert_eq!(header.stream_id, 0);
    }


}
