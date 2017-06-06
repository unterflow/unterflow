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
    pub version: u16,
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
    fn test_decode_gossip() {
        cursor!(reader, "gossip");

        let header = MessageHeader::from_bytes(&mut reader).unwrap();
        assert_eq!(header, Gossip::message_header());

        let request = Gossip::from_bytes(&mut reader).unwrap();
        assert_eq!(header, MessageHeader::from(&request));
        assert_eq!(request.peers,
                   vec![Peer {
                            state: PeerState::Alive,
                            generation: 1496752869673,
                            version: 394,
                            endpoints: vec![Endpoint {
                                                endpoint_type: EndpointType::Client,
                                                port: 8647,
                                                host: "localhost".to_string(),
                                            },
                                            Endpoint {
                                                endpoint_type: EndpointType::Management,
                                                port: 8648,
                                                host: "localhost".to_string(),
                                            },
                                            Endpoint {
                                                endpoint_type: EndpointType::Replication,
                                                port: 8649,
                                                host: "localhost".to_string(),
                                            }],
                            raf_memberships: vec![RaftMembership {
                                                      partition_id: 0,
                                                      term: 1,
                                                      state: RaftMembershipState::Leader,
                                                      topic_name: "default-topic".to_string(),
                                                  }],
                        },
                        Peer {
                            state: PeerState::Alive,
                            generation: 1496752878665,
                            version: 384,
                            endpoints: vec![Endpoint {
                                                endpoint_type: EndpointType::Client,
                                                port: 9647,
                                                host: "localhost".to_string(),
                                            },
                                            Endpoint {
                                                endpoint_type: EndpointType::Management,
                                                port: 9648,
                                                host: "localhost".to_string(),
                                            },
                                            Endpoint {
                                                endpoint_type: EndpointType::Replication,
                                                port: 9649,
                                                host: "localhost".to_string(),
                                            }],
                            raf_memberships: vec![RaftMembership {
                                                      partition_id: 0,
                                                      term: 1,
                                                      state: RaftMembershipState::Follower,
                                                      topic_name: "default-topic".to_string(),
                                                  }],
                        }]);
    }

}
