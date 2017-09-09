

use bytes::{BufMut, BytesMut};
use futures::Future;

use std::{io, str};
use std::net::SocketAddr;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::TcpClient;
use tokio_proto::multiplex::{ClientProto, ClientService, RequestId};
use tokio_service::Service;

use unterflow_protocol::TransportMessage;
use unterflow_protocol::frame::DataFrameHeader;
use unterflow_protocol::io::{FromBytes, HasBlockLength, ToBytes};

pub struct MultiplexedClient {
    inner: ClientService<TcpStream, MultiplexedProto>,
}

impl MultiplexedClient {
    pub fn connect(
        addr: &SocketAddr,
        handle: &Handle,
    ) -> Box<Future<Item = MultiplexedClient, Error = io::Error>> {
        Box::new(TcpClient::new(MultiplexedProto).connect(addr, handle).map(
            |service| MultiplexedClient { inner: service },
        ))
    }
}

impl Service for MultiplexedClient {
    type Request = TransportMessage;
    type Response = TransportMessage;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        Box::new(self.inner.call(request))
    }
}

struct MultiplexedCodec {
    header: Option<DataFrameHeader>,
}

impl MultiplexedCodec {
    fn decode_frame(
        &mut self,
        header: DataFrameHeader,
        buffer: &mut BytesMut,
    ) -> Result<Option<(RequestId, TransportMessage)>, io::Error> {
        let frame_length = header.aligned_length() - DataFrameHeader::block_length() as usize;

        if buffer.len() < frame_length {
            self.header = Some(header);
            Ok(None)
        } else {
            let frame = buffer.split_to(frame_length);
            let mut reader = io::Cursor::new(frame);
            let frame = TransportMessage::read(header, &mut reader)?;

            let request_id = match frame {
                TransportMessage::RequestResponse(ref r) => r.request_header.request_id,
                r => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Expected request response message but got {:?}", r),
                    ))
                }
            };

            Ok(Some((request_id as RequestId, frame)))
        }
    }
}

impl Decoder for MultiplexedCodec {
    type Item = (RequestId, TransportMessage);
    type Error = io::Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(header) = self.header.take() {
            self.decode_frame(header, buffer)
        } else {
            let header_length = DataFrameHeader::block_length() as usize;
            if buffer.len() < header_length {
                Ok(None)
            } else {
                let header = buffer.split_to(header_length);
                let mut reader = io::Cursor::new(header);
                let header = DataFrameHeader::from_bytes(&mut reader)?;
                self.decode_frame(header, buffer)
            }
        }
    }
}

impl Encoder for MultiplexedCodec {
    type Item = (RequestId, TransportMessage);
    type Error = io::Error;

    fn encode(&mut self, request: Self::Item, buffer: &mut BytesMut) -> Result<(), io::Error> {
        let (request_id, mut request) = request;

        match request {
            TransportMessage::RequestResponse(ref mut r) => {
                r.request_header.request_id = request_id;
            }
            r => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Expected request response message but got {:?}", r),
                ))
            }
        };

        let length = request.length();

        if buffer.remaining_mut() < length {
            buffer.reserve(length);
        }

        let mut writer = buffer.writer();

        request.to_bytes(&mut writer)
    }
}

struct MultiplexedProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for MultiplexedProto {
    type Request = TransportMessage;
    type Response = TransportMessage;
    type Transport = Framed<T, MultiplexedCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MultiplexedCodec { header: None }))
    }
}
