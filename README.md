# unterflow
Rust implementation of a to be announced protocol

## unterflow-dump

Dump unterflow network traffic.

### Usage

```bash
$ unterflow-dump --help
unterflow-dump 0.1.0
Dump unterflow protocol packages

USAGE:
    unterflow-dump [FLAGS] [OPTIONS] --interface <interface>

FLAGS:
    -h, --help       Prints help information
    -v               Enable logging, use multiple `v`s to increase verbosity
    -V, --version    Prints version information

OPTIONS:
    -i, --interface <interface>    Interface to capture
    -p, --port <port>              Port to capture [default: 51015]
```

### Example

```bash
$ sudo unterflow-dump -i lo

==>  Packet: 127.0.0.1:40198 -> 127.0.0.1:51015 (160 bytes; seq: 68493699)
FrameHeader { length: 147, version: 0, flags: 0, type_id: Message, stream_id: 2 }
TransportHeader { protocol_id: RequestResponse }
RequestResponseHeader { connection_id: 1, request_id: 0 }
MessageHeader { block_length: 11, template_id: 20, schema_id: 0, version: 1 }
ExecuteCommandRequest { partition_id: 0, key: 18446744073709551615, event_type: TaskEvent, topic_name: "default-topic", command: Data([133, 169, 101, 118, 101, 110, 116, 84, 121, 112, 101, 166, 67, 82, 69, 65, 84, 69, 167, 114, 101, 116, 114, 105, 101, 115, 3, 164, 116, 121, 112, 101, 163, 102, 111, 111, 167, 104, 101, 97, 100, 101, 114, 115, 130, 162, 107, 49, 161, 97, 162, 107, 50, 161, 98, 167, 112, 97, 121, 108, 111, 97, 100, 196, 10, 129, 167, 112, 97, 121, 108, 111, 97, 100, 123]) }

==>  Packet: 127.0.0.1:51015 -> 127.0.0.1:40198 (288 bytes; seq: 417626964)
FrameHeader { length: 269, version: 0, flags: 0, type_id: Message, stream_id: 2 }
TransportHeader { protocol_id: RequestResponse }
RequestResponseHeader { connection_id: 1, request_id: 0 }
MessageHeader { block_length: 10, template_id: 21, schema_id: 0, version: 1 }
ExecuteCommandResponse { partition_id: 0, key: 4294967544, topic_name: "default-topic", event: Data([135, 169, 101, 118, 101, 110, 116, 84, 121, 112, 101, 167, 67, 82, 69, 65, 84, 69, 68, 168, 108, 111, 99, 107, 84, 105, 109, 101, 211, 128, 0, 0, 0, 0, 0, 0, 0, 169, 108, 111, 99, 107, 79, 119, 110, 101, 114, 255, 167, 114, 101, 116, 114, 105, 101, 115, 3, 164, 116, 121, 112, 101, 163, 102, 111, 111, 167, 104, 101, 97, 100, 101, 114, 115, 136, 173, 98, 112, 109, 110, 80, 114, 111, 99, 101, 115, 115, 73, 100, 160, 185, 119, 111, 114, 107, 102, 108, 111, 119, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 86, 101, 114, 115, 105, 111, 110, 255, 179, 119, 111, 114, 107, 102, 108, 111, 119, 73, 110, 115, 116, 97, 110, 99, 101, 75, 101, 121, 255, 170, 97, 99, 116, 105, 118, 105, 116, 121, 73, 100, 160, 179, 97, 99, 116, 105, 118, 105, 116, 121, 73, 110, 115, 116, 97, 110, 99, 101, 75, 101, 121, 255, 173, 99, 117, 115, 116, 111, 109, 72, 101, 97, 100, 101, 114, 115, 144, 162, 107, 49, 161, 97, 162, 107, 50, 161, 98, 167, 112, 97, 121, 108, 111, 97, 100, 196, 10, 129, 167, 112, 97, 121, 108, 111, 97, 100, 123]) }

==>  Packet: 127.0.0.1:40198 -> 127.0.0.1:51015 (16 bytes; seq: 68493859)
FrameHeader { length: 0, version: 0, flags: 0, type_id: ControlClose, stream_id: 0 }

==>  Packet: 127.0.0.1:51015 -> 127.0.0.1:40198 (16 bytes; seq: 417627252)
FrameHeader { length: 0, version: 0, flags: 0, type_id: ControlEndOfStream, stream_id: 0 }
```
