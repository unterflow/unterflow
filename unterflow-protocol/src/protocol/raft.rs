use convert::*;
use errors::*;


#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Member {
    port: u16,
    host: String,
}


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "4", version = "1")]
pub struct JoinRequest {
    partition_id: u16,
    term: u16,
    port: u16,
    topic_name: String,
    host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "1", schema_id = "4", version = "1")]
pub struct JoinResponse {
    term: u16,
    succeeded: bool,
    configuration_entry_position: u64,
    configuration_entry_term: i32,
    members: Vec<Member>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "2", schema_id = "4", version = "1")]
pub struct LeaveRequest {
    partition_id: u16,
    term: u16,
    port: u16,
    topic_name: String,
    host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "3", schema_id = "4", version = "1")]
pub struct LeaveResponse {
    term: u16,
    succeeded: bool,
    configuration_entry_position: u64,
    configuration_entry_term: i32,
    members: Vec<Member>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "4", schema_id = "4", version = "1")]
pub struct ConfigurationRequest {
    partition_id: u16,
    term: u16,
    configuration_entry_position: u64,
    configuration_entry_term: i32,
    members: Vec<Member>,
    topic_name: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "5", schema_id = "4", version = "1")]
pub struct ConfigurationResponse {
    term: u16,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "6", schema_id = "4", version = "1")]
pub struct PollRequest {
    partition_id: u16,
    term: u16,
    last_entry_position: u64,
    last_entry_term: i32,
    port: u16,
    topic_name: String,
    host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "7", schema_id = "4", version = "1")]
pub struct PollResponse {
    term: u16,
    granted: bool,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "8", schema_id = "4", version = "1")]
pub struct VoteRequest {
    partition_id: u16,
    term: u16,
    last_entry_position: u64,
    last_entry_term: i32,
    port: u16,
    topic_name: String,
    host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "9", schema_id = "4", version = "1")]
pub struct VoteResponse {
    term: u16,
    granted: bool,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "10", schema_id = "4", version = "1")]
pub struct AppendRequest {
    partition_id: u16,
    term: u16,
    previous_entry_position: u64,
    previous_entry_term: i32,
    commit_position: u64,
    port: u16,
    topic_name: String,
    host: String,
    data: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "10", schema_id = "4", version = "1")]
pub struct AppendResponse {
    partition_id: u16,
    term: u16,
    succeeded: bool,
    entry_position: u64,
    port: u16,
    topic_name: String,
    host: String,
}
