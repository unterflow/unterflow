use std::io::{Cursor, Read};
use unterflow_protocol::convert::*;
use unterflow_protocol::protocol::client::*;
use unterflow_protocol::protocol::gossip::*;
use unterflow_protocol::protocol::management::*;
use unterflow_protocol::protocol::raft::*;
use unterflow_protocol::protocol::transport::*;
use network::CapturedPacket;
use errors::*;

pub fn dump_packet(packet: &CapturedPacket) -> Result<()> {
    println!();
    println!("==>  Packet: {}", packet);

    let mut payload = Cursor::new(packet.payload());

    let frame = FrameHeader::from_bytes(&mut payload)?;
    println!("{:?}", frame);

    match frame.type_id {
        FrameType::Message => dump_message(&mut payload),
        FrameType::Unknown => bail!("Unknown frame type: {:?}", frame),
        _ => Ok(()),
    }
}

pub fn dump_message(mut payload: &mut Read) -> Result<()> {
    let transport = TransportHeader::from_bytes(&mut payload)?;
    println!("{:?}", transport);
    match transport.protocol_id {
        TransportProtocol::RequestResponse => {
            let header = RequestResponseHeader::from_bytes(&mut payload)?;
            println!("{:?}", header);
        }
        TransportProtocol::FullDuplexSingleMessage => {
            let header = SingleMessageHeader::from_bytes(&mut payload)?;
            println!("{:?}", header);
        }
        TransportProtocol::Unknown => bail!("Unknown transport protocol: {:?}", transport),
    }

    let header = MessageHeader::from_bytes(&mut payload)?;
    println!("{:?}", header);

    // Client Protocol
    if header == ErrorResponse::message_header() {
        let message = ErrorResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == ControlMessageRequest::message_header() {
        let message = ControlMessageRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == ControlMessageResponse::message_header() {
        let message = ControlMessageResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == ExecuteCommandRequest::message_header() {
        let message = ExecuteCommandRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == ExecuteCommandResponse::message_header() {
        let message = ExecuteCommandResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == SubscribedEvent::message_header() {
        let message = SubscribedEvent::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == BrokerEventMetadata::message_header() {
        let message = BrokerEventMetadata::from_bytes(&mut payload)?;
        println!("{:?}", message);
    }
    // Management Protocol
    else if header == InvitationRequest::message_header() {
        let message = InvitationRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == InvitationResponse::message_header() {
        let message = InvitationResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    }
    // Raft Protocol
    else if header == JoinRequest::message_header() {
        let message = JoinRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == JoinResponse::message_header() {
        let message = JoinResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == LeaveRequest::message_header() {
        let message = LeaveRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == LeaveResponse::message_header() {
        let message = LeaveResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == ConfigurationRequest::message_header() {
        let message = ConfigurationRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == ConfigurationResponse::message_header() {
        let message = ConfigurationResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == PollRequest::message_header() {
        let message = PollRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == PollResponse::message_header() {
        let message = PollResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == VoteRequest::message_header() {
        let message = VoteRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == VoteResponse::message_header() {
        let message = VoteResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == AppendRequest::message_header() {
        let message = AppendRequest::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == AppendResponse::message_header() {
        let message = AppendResponse::from_bytes(&mut payload)?;
        println!("{:?}", message);
    }
    // Gossip Protocol
    else if header == Gossip::message_header() {
        let message = Gossip::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == Probe::message_header() {
        let message = Probe::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else if header == PeerDescriptor::message_header() {
        let message = PeerDescriptor::from_bytes(&mut payload)?;
        println!("{:?}", message);
    } else {
        bail!("Unknown message header: {:?}", header);
    }

    Ok(())
}
