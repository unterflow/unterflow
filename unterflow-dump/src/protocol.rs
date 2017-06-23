use errors::*;
use std::fmt;
use std::io::{Read, Seek, SeekFrom};
use unterflow_protocol::convert::*;
use unterflow_protocol::protocol::client::*;
use unterflow_protocol::protocol::gossip::*;
use unterflow_protocol::protocol::management::*;
use unterflow_protocol::protocol::raft::*;
use unterflow_protocol::protocol::transport::*;

#[derive(Default, Debug)]
pub struct Protocol {
    pub frame: Option<FrameHeader>,
    transport: Option<TransportHeader>,
    protocol: Option<Box<fmt::Debug>>,
    header: Option<MessageHeader>,
    message: Option<Box<fmt::Debug>>,
    pretty: bool,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref frame) = self.frame {
            writeln!(f, "{:?}", frame)?;
        }
        if let Some(ref transport) = self.transport {
            writeln!(f, "{:?}", transport)?;
        }
        if let Some(ref protocol) = self.protocol {
            writeln!(f, "{:?}", protocol)?;
        }
        if let Some(ref header) = self.header {
            writeln!(f, "{:?}", header)?;
        }
        if let Some(ref message) = self.message {
            if self.pretty {
                writeln!(f, "{:#?}", message)?;
            } else {
                writeln!(f, "{:?}", message)?;
            }
        }

        Ok(())
    }
}

impl Protocol {
    pub fn parse<T: Read + Seek>(mut payload: &mut T, offset: u64, pretty: bool) -> Result<Self> {
        payload.seek(SeekFrom::Start(offset))?;
        let mut protocol = Protocol::default();
        protocol.pretty = pretty;

        if let Ok(frame) = FrameHeader::from_bytes(&mut payload) {

            if frame.type_id == FrameType::Unknown {
                bail!("Unknown frame type: {:?}", frame);
            }

            if frame.type_id == FrameType::Message {

                let transport = TransportHeader::from_bytes(&mut payload)?;
                protocol.protocol = match transport.protocol_id {
                    TransportProtocol::RequestResponse => {
                        Some(Box::new(RequestResponseHeader::from_bytes(&mut payload)?))
                    }
                    TransportProtocol::FullDuplexSingleMessage => {
                        Some(Box::new(SingleMessageHeader::from_bytes(&mut payload)?))
                    }
                    TransportProtocol::Unknown => bail!("Unknown transport protocol: {:?}", transport),
                };
                protocol.transport = Some(transport);

                let header = MessageHeader::from_bytes(&mut payload)?;

                // Client Protocol
                if header == ErrorResponse::message_header() {
                    protocol.message = Some(Box::new(ErrorResponse::from_bytes(&mut payload)?));
                } else if header == ControlMessageRequest::message_header() {
                    protocol.message = Some(Box::new(ControlMessageRequest::from_bytes(&mut payload)?));
                } else if header == ControlMessageResponse::message_header() {
                    protocol.message = Some(Box::new(ControlMessageResponse::from_bytes(&mut payload)?));
                } else if header == ExecuteCommandRequest::message_header() {
                    protocol.message = Some(Box::new(ExecuteCommandRequest::from_bytes(&mut payload)?));
                } else if header == ExecuteCommandResponse::message_header() {
                    protocol.message = Some(Box::new(ExecuteCommandResponse::from_bytes(&mut payload)?));
                } else if header == SubscribedEvent::message_header() {
                    protocol.message = Some(Box::new(SubscribedEvent::from_bytes(&mut payload)?));
                } else if header == BrokerEventMetadata::message_header() {
                    protocol.message = Some(Box::new(BrokerEventMetadata::from_bytes(&mut payload)?));
                }
                // Management Protocol
                else if header == InvitationRequest::message_header() {
                    protocol.message = Some(Box::new(InvitationRequest::from_bytes(&mut payload)?));
                } else if header == InvitationResponse::message_header() {
                    protocol.message = Some(Box::new(InvitationResponse::from_bytes(&mut payload)?));
                }
                // Raft Protocol
                else if header == JoinRequest::message_header() {
                    protocol.message = Some(Box::new(JoinRequest::from_bytes(&mut payload)?));
                } else if header == JoinResponse::message_header() {
                    protocol.message = Some(Box::new(JoinResponse::from_bytes(&mut payload)?));
                } else if header == LeaveRequest::message_header() {
                    protocol.message = Some(Box::new(LeaveRequest::from_bytes(&mut payload)?));
                } else if header == LeaveResponse::message_header() {
                    protocol.message = Some(Box::new(LeaveResponse::from_bytes(&mut payload)?));
                } else if header == ConfigurationRequest::message_header() {
                    protocol.message = Some(Box::new(ConfigurationRequest::from_bytes(&mut payload)?));
                } else if header == ConfigurationResponse::message_header() {
                    protocol.message = Some(Box::new(ConfigurationResponse::from_bytes(&mut payload)?));
                } else if header == PollRequest::message_header() {
                    protocol.message = Some(Box::new(PollRequest::from_bytes(&mut payload)?));
                } else if header == PollResponse::message_header() {
                    protocol.message = Some(Box::new(PollResponse::from_bytes(&mut payload)?));
                } else if header == VoteRequest::message_header() {
                    protocol.message = Some(Box::new(VoteRequest::from_bytes(&mut payload)?));
                } else if header == VoteResponse::message_header() {
                    protocol.message = Some(Box::new(VoteResponse::from_bytes(&mut payload)?));
                } else if header == AppendRequest::message_header() {
                    protocol.message = Some(Box::new(AppendRequest::from_bytes(&mut payload)?));
                } else if header == AppendResponse::message_header() {
                    protocol.message = Some(Box::new(AppendResponse::from_bytes(&mut payload)?));
                }
                // Gossip Protocol
                else if header == Gossip::message_header() {
                    protocol.message = Some(Box::new(Gossip::from_bytes(&mut payload)?));
                } else if header == Probe::message_header() {
                    protocol.message = Some(Box::new(Probe::from_bytes(&mut payload)?));
                } else if header == PeerDescriptor::message_header() {
                    protocol.message = Some(Box::new(PeerDescriptor::from_bytes(&mut payload)?));
                } else {
                    bail!("Unknown message header: {:?}", header);
                }

                protocol.header = Some(header);
            }

            protocol.frame = Some(frame);
        }

        Ok(protocol)
    }
}
