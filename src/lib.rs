extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;
extern crate unterflow_protocol;

mod proto;

use futures::Future;
use proto::MultiplexedClient;
use std::collections::BTreeMap;
use std::io;
use std::net::SocketAddr;
use tokio_core::reactor::Handle;
use tokio_service::Service;

use unterflow_protocol::{RequestResponseMessage, TransportMessage};
use unterflow_protocol::io::FromData;
use unterflow_protocol::message::{SocketAddress, TopicLeader, TopologyRequest, TopologyResponse};
use unterflow_protocol::sbe::ControlMessageType;

pub struct Client {
    client: MultiplexedClient,
}

impl Client {
    pub fn connect(
        addr: &SocketAddr,
        handle: &Handle,
    ) -> Box<Future<Item = Client, Error = io::Error>> {
        Box::new(MultiplexedClient::connect(addr, handle).map(|client| {
            Client { client }
        }))
    }

    pub fn topology(&self) -> Box<Future<Item = Topology, Error = io::Error>> {
        let message = ControlMessageType::RequestTopology
            .with(&TopologyRequest {})
            .expect("Failed to create message");

        let request = TransportMessage::request(1, message);

        let request = self.client.call(request).and_then(|response| {
            if let TransportMessage::RequestResponse(response) = response {
                if let RequestResponseMessage::ControlMessageResponse(ref message) =
                    *response.message()
                {
                    let topology =
                        TopologyResponse::from_data(message).expect("Failed to read topology");
                    Ok(topology.into())
                } else {
                    Err(other(format!(
                        "Unexpected response message {:?}",
                        response.message()
                    )))
                }
            } else {
                Err(other(format!("Unexpected response {:?}", response)))
            }
        });

        Box::new(request)
    }
}

fn other(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

#[derive(Debug)]
pub struct Topology {
    topic_leaders: BTreeMap<String, BTreeMap<u16, Broker>>,
    brokers: Vec<Broker>,
}

impl From<TopologyResponse> for Topology {
    fn from(response: TopologyResponse) -> Self {
        let mut topic_leaders = BTreeMap::new();

        for leader in response.topic_leaders {
            topic_leaders
                .entry(leader.topic_name.clone())
                .or_insert_with(BTreeMap::new)
                .insert(leader.port, leader.into());
        }

        let brokers = response.brokers.into_iter().map(|b| b.into()).collect();
        Topology {
            topic_leaders,
            brokers,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Broker {
    host: String,
    port: u16,
}

impl From<SocketAddress> for Broker {
    fn from(address: SocketAddress) -> Self {
        Broker {
            host: address.host,
            port: address.port,
        }
    }
}

impl From<TopicLeader> for Broker {
    fn from(leader: TopicLeader) -> Self {
        Broker {
            host: leader.host,
            port: leader.port,
        }
    }
}
