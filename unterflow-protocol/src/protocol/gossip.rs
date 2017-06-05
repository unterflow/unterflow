use convert::*;
use errors::*;


#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum PeerState {
    Alive,
    Suspect,
    Dead,
    Unknown,
}

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum EndpointType {
    Client,
    Management,
    Replication,
    Unknown,
}

#[derive(Debug, PartialEq, EnumDefault, FromBytes, BlockLength)]
pub enum RaftMembershipState {
    Inactive,
    Follower,
    Candidate,
    Leader,
    Unknown,
}


#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Peer {
    pub state: PeerState,
    pub generation: u64,
    pub version: u64,
    pub endpoints: Vec<Endpoint>,
    pub raf_memberships: Vec<RaftMembership>,
}

#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Endpoint {
    pub endpoint_type: EndpointType,
    pub port: u16,
    pub host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct RaftMembership {
    pub partition_id: u16,
    pub term: u16,
    pub state: RaftMembershipState,
    pub topic_name: String,
}


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "3", version = "1")]
pub struct Gossip {
    pub peers: Vec<Peer>,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "1", schema_id = "3", version = "1")]
pub struct Probe {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "100", schema_id = "3", version = "1")]
pub struct PeerDescriptor {
    pub state: PeerState,
    pub generation: u64,
    pub version: u16,
    pub change_state_time: u64,
    pub endpoints: Vec<Endpoint>,
    pub raft_memberships: Vec<RaftMembership>,
}
