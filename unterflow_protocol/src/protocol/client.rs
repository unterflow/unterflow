use convert::*;
use errors::*;

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum ErrorCode {
    MessageNotSupported,
    TopicNotFound,
    RequestWriteFailure,
    InvalidClientVersion,
    RequestTimeout,
    RequestProcessingFailure,
    Unknown,
}
#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum EventType {
    TaskEvent,
    RaftEvent,
    SubscriptionEvent,
    SubscriberEvent,
    DeploymentEvent,
    WorkflowEvent,
    IncidentEvent,
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

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum SubscriptionType {
    TaskSubscription,
    TopicSubscription,
    Unknown,
}


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "0", version = "1")]
pub struct ErrorResponse {
    error_code: ErrorCode,
    error_data: Data,
    failed_request: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "10", schema_id = "0", version = "1")]
pub struct ControlMessageRequest {
    message_type: ControlMessageType,
    data: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "11", schema_id = "0", version = "1")]
pub struct ControlMessageResponse {
    data: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "20", schema_id = "0", version = "1")]
pub struct ExecuteCommandRequest {
    partition_id: u16,
    key: u64,
    event_type: EventType,
    topic_name: String,
    command: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "21", schema_id = "0", version = "1")]
pub struct ExecuteCommandResponse {
    partition_id: u16,
    key: u64,
    topic_name: String,
    event: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "30", schema_id = "0", version = "1")]
pub struct SubscribedEvent {
    partition_id: u16,
    position: u64,
    key: u64,
    subscriber_key: u64,
    subscription_type: SubscriptionType,
    event_type: EventType,
    topic_name: String,
    event: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "200", schema_id = "0", version = "1")]
pub struct BrokerEventMetadata {
    req_channel_id: i32,
    req_connection_id: u64,
    req_request_id: u64,
    raft_term_id: i32,
    subscription_id: u64,
    protocol_version: u16,
    event_type: EventType,
    incident_key: u64,
}


#[cfg(test)]
mod tests {

    use super::*;
    use protocol::transport::*;

    use std::io::Cursor;

    macro_rules! cursor {
        ($reader:ident, $file:expr) => (
            let data = include_bytes!(concat!("../../../dumps/", $file)).to_vec();
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
        cursor!(reader, "create-task-request");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, ExecuteCommandRequest::message_header());

        let request = ExecuteCommandRequest::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.partition_id, 0);
        assert_eq!(request.key, ::std::u64::MAX);
        assert_eq!(request.event_type, EventType::TaskEvent);
        assert_eq!(request.topic_name, "default-topic");
        assert_eq!(request.command.len(), 75);
    }

    #[test]
    fn test_decode_create_task_response() {
        cursor!(reader, "create-task-response");

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
        cursor!(reader, "close-subscription-request");

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
        cursor!(reader, "close-subscription-response");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, ControlMessageResponse::message_header());

        let request = ControlMessageResponse::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.data.len(), 61);
    }

}
