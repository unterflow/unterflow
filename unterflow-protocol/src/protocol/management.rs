use convert::*;
use errors::*;


#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Member {
    port: u16,
    host: String,
}


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "5", version = "1")]
pub struct InvitationRequest {
    partition_id: u16,
    term: u16,
    name: u16,
    members: Vec<Member>,
    topic_name: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "1", schema_id = "5", version = "1")]
pub struct InvitationResponse {
    term: u16,
}
