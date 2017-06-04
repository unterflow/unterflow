use convert::*;
use errors::*;


#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Member {
    pub port: u16,
    pub host: String,
}


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "5", version = "1")]
pub struct InvitationRequest {
    pub partition_id: u16,
    pub term: u16,
    pub name: u16,
    pub members: Vec<Member>,
    pub topic_name: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "1", schema_id = "5", version = "1")]
pub struct InvitationResponse {
    pub term: u16,
}
