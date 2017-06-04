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


#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "0", schema_id = "3", version = "1")]
pub struct Gossip {
    peers: Vec<Peer>,
}

#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Peer {
    state: PeerState,
    generation: u64,
    version: u64,
    endpoints: Vec<Endpoint>,
    raf_memberships: Vec<RaftMembership>,
}

#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct Endpoint {
    endpoint_type: EndpointType,
    port: u16,
    host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes)]
pub struct RaftMembership {
    partition_id: u16,
    term: u16,
    state: RaftMembershipState,
    topic_name: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "1", schema_id = "3", version = "1")]
pub struct Probe {
    port: u16,
    host: String,
}

#[derive(Debug, PartialEq, Default, FromBytes, BlockLength, Message)]
#[message(template_id = "100", schema_id = "3", version = "1")]
pub struct PeerDescriptor {
    state: PeerState,
    generation: u64,
    version: u16,
    change_state_time: u64,
    endpoints: Vec<Endpoint>,
    raft_memberships: Vec<RaftMembership>,
}

