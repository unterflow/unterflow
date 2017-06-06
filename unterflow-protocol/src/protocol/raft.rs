use convert::*;
use errors::*;


#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Member {
    pub port: u16,
    pub host: String,
}


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "4", version = "1")]
pub struct JoinRequest {
    pub partition_id: u16,
    pub term: u16,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "1", schema_id = "4", version = "1")]
pub struct JoinResponse {
    pub term: u16,
    pub succeeded: bool,
    pub configuration_entry_position: u64,
    pub configuration_entry_term: i32,
    pub members: Vec<Member>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "2", schema_id = "4", version = "1")]
pub struct LeaveRequest {
    pub partition_id: u16,
    pub term: u16,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "3", schema_id = "4", version = "1")]
pub struct LeaveResponse {
    pub term: u16,
    pub succeeded: bool,
    pub configuration_entry_position: u64,
    pub configuration_entry_term: i32,
    pub members: Vec<Member>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "4", schema_id = "4", version = "1")]
pub struct ConfigurationRequest {
    pub partition_id: u16,
    pub term: u16,
    pub configuration_entry_position: u64,
    pub configuration_entry_term: i32,
    pub members: Vec<Member>,
    pub topic_name: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "5", schema_id = "4", version = "1")]
pub struct ConfigurationResponse {
    pub term: u16,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "6", schema_id = "4", version = "1")]
pub struct PollRequest {
    pub partition_id: u16,
    pub term: u16,
    pub last_entry_position: u64,
    pub last_entry_term: i32,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "7", schema_id = "4", version = "1")]
pub struct PollResponse {
    pub term: u16,
    pub granted: bool,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "8", schema_id = "4", version = "1")]
pub struct VoteRequest {
    pub partition_id: u16,
    pub term: u16,
    pub last_entry_position: u64,
    pub last_entry_term: i32,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "9", schema_id = "4", version = "1")]
pub struct VoteResponse {
    pub term: u16,
    pub granted: bool,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "10", schema_id = "4", version = "1")]
pub struct AppendRequest {
    pub partition_id: u16,
    pub term: u16,
    pub previous_entry_position: u64,
    pub previous_entry_term: i32,
    pub commit_position: u64,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
    pub data: Data,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "11", schema_id = "4", version = "1")]
pub struct AppendResponse {
    pub partition_id: u16,
    pub term: u16,
    pub succeeded: bool,
    pub entry_position: u64,
    pub port: u16,
    pub topic_name: String,
    pub host: String,
}
