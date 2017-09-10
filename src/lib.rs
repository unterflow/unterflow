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
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use tokio_core::reactor::Handle;
use tokio_service::Service;

use unterflow_protocol::{RequestResponseMessage, TransportMessage};
use unterflow_protocol::io::{FromData, ToData};
use unterflow_protocol::message::{CREATE_STATE, NIL, SocketAddress, TaskEvent, TopicLeader,
                                  TopologyRequest, TopologyResponse};
use unterflow_protocol::sbe::{ControlMessageType, EventType, ExecuteCommandRequest};

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

    pub fn create_task(
        &self,
        topic: &str,
        event: TaskEvent,
    ) -> Box<Future<Item = Task, Error = io::Error>> {
        let command = event.to_data().expect("Failed to conver event");

        let message = ExecuteCommandRequest {
            topic_name: topic.into(),
            partition_id: 0,
            position: u64::max_value(),
            key: u64::max_value(),
            event_type: EventType::TaskEvent,
            command,
        };

        let request = TransportMessage::request(0, message);

        let request = self.client.call(request).and_then(|response| {
            if let TransportMessage::RequestResponse(response) = response {
                if let RequestResponseMessage::ExecuteCommandResponse(ref message) =
                    *response.message()
                {
                    let key = message.key;
                    let task = TaskEvent::from_data(message).expect("Failed to read task");
                    Ok(Task {
                        state: task.state,
                        key,
                    })
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

    pub fn new_task(&self, task_type: String) -> TaskBuilder {
        TaskBuilder {
            client: self,
            task_type,
            retires: 3,
            header: HashMap::new(),
            payload: NIL.to_vec(),
        }
    }
}

pub struct TaskBuilder<'a> {
    client: &'a Client,
    task_type: String,
    retires: i32,
    header: HashMap<String, String>,
    payload: Vec<u8>,
}

impl<'a> TaskBuilder<'a> {
    pub fn retires(mut self, retries: i32) -> Self {
        self.retires = retries;
        self
    }

    pub fn add_header(mut self, key: String, value: String) -> Self {
        self.header.insert(key, value);
        self
    }

    pub fn payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }

    pub fn create(&self, topic: &str) -> Box<Future<Item = Task, Error = io::Error>> {
        let event = TaskEvent {
            state: CREATE_STATE.into(),
            task_type: self.task_type.clone(),
            retries: self.retires,
            payload: self.payload.clone().into(),
            ..Default::default()
        };

        self.client.create_task(topic, event)
    }
}

fn other(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

#[derive(Debug)]
pub struct Topology {
    pub topic_leaders: HashMap<String, HashMap<u16, Broker>>,
    pub brokers: Vec<Broker>,
}

impl From<TopologyResponse> for Topology {
    fn from(response: TopologyResponse) -> Self {
        let mut topic_leaders = HashMap::new();

        for leader in response.topic_leaders {
            topic_leaders
                .entry(leader.topic_name.clone())
                .or_insert_with(HashMap::new)
                .insert(leader.partition_id, leader.into());
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
    pub host: String,
    pub port: u16,
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

#[derive(Debug)]
pub struct Task {
    pub state: String,
    pub key: u64,
}
