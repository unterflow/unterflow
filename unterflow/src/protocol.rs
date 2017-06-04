use convert::*;
use errors::*;

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength)]
pub struct MessageHeader {
    block_length: u16,
    template_id: u16,
    schema_id: u16,
    version: u16,
}

impl<'a, T: Message + BlockLength> From<&'a T> for MessageHeader {
    fn from(_: &'a T) -> Self {
        T::message_header()
    }
}

impl<T: Message + BlockLength> ToMessageHeader for T {
    fn message_header() -> MessageHeader {
        MessageHeader {
            block_length: T::block_length(),
            template_id: T::template_id(),
            schema_id: T::schema_id(),
            version: T::version(),
        }
    }
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "20", schema_id = "0", version = "1")]
pub struct ExecuteCommandRequest {
    partition_id: u16,
    key: u64,
    event_type: EventType,
    topic_name: String,
    command: Vec<u8>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "21", schema_id = "0", version = "1")]
pub struct ExecuteCommandResponse {
    partition_id: u16,
    key: u64,
    topic_name: String,
    event: Vec<u8>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "10", schema_id = "0", version = "1")]
pub struct ControlMessageRequest {
    message_type: ControlMessageType,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "11", schema_id = "0", version = "1")]
pub struct ControlMessageResponse {
    data: Vec<u8>,
}


#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum EventType {
    Task,
    Raft,
    Subscription,
    Subscriber,
    Deployment,
    Workflow,
    Incident,
    Unknown,
}

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum ControlMessageType {
    AddTaskSubscription,
    RemoveTaskSubscription,
    IncreaseTaskSubscriptionCredits,
    RemoveTopicSubscription,
    Unknown,
}


#[cfg(test)]
mod tests {

    use super::*;
    use transport::*;

    use std::io::Cursor;

    macro_rules! cursor {
        ($reader:ident, $file:expr) => (
            let data = include_bytes!($file).to_vec();
            let mut $reader = Cursor::new(data);

            FrameHeader::skip_block(&mut $reader).unwrap();
            TransportHeader::skip_block(&mut $reader).unwrap();
            RequestResponseHeader::skip_block(&mut $reader).unwrap();
        )
    }

    #[test]
    fn test_block_length() {
        assert_eq!(MessageHeader::block_length(), 8);
        assert_eq!(ExecuteCommandRequest::block_length(), 11);
        assert_eq!(EventType::block_length(), 1);
    }

    #[test]
    fn test_decode_create_task_request() {
        cursor!(reader, "../../dumps/create-task-request");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, ExecuteCommandRequest::message_header());

        let request = ExecuteCommandRequest::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.partition_id, 0);
        assert_eq!(request.key, ::std::u64::MAX);
        assert_eq!(request.event_type, EventType::Task);
        assert_eq!(request.topic_name, "default-topic");
        assert_eq!(request.command.len(), 75);
    }

    #[test]
    fn test_decode_create_task_response() {
        cursor!(reader, "../../dumps/create-task-response");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, ExecuteCommandResponse::message_header());

        let request = ExecuteCommandResponse::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.partition_id, 0);
        assert_eq!(request.key, 4294967544);
        assert_eq!(request.topic_name, "default-topic");
        assert_eq!(request.event.len(), 216);
    }

    #[test]
    fn test_decode_close_subscription_request() {
        cursor!(reader, "../../dumps/close-subscription-request");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, ControlMessageRequest::message_header());

        let request = ControlMessageRequest::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.message_type,
                   ControlMessageType::RemoveTopicSubscription);
        assert_eq!(request.data.len(), 61);
    }

    #[test]
    fn test_decode_close_subscription_response() {
        cursor!(reader, "../../dumps/close-subscription-response");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, ControlMessageResponse::message_header());

        let request = ControlMessageResponse::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.data.len(), 61);
    }

}
