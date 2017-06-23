# unterflow

[![Build Status](https://travis-ci.org/menski/unterflow.svg?branch=master)](https://travis-ci.org/menski/unterflow)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/menski/unterflow?branch=master&svg=true)](https://ci.appveyor.com/project/menski/unterflow)

Rust implementation of a to be announced protocol

## unterflow-dump

Dump unterflow network traffic.

### Install

Download latest release from [release page](https://github.com/menski/unterflow/releases) or build locally using:

```bash
$ cargo install --path unterflow-dump
```

#### Capabilities

To capture network packages `unterflow-dump` requires access to raw sockets.
Therefore it has to be run with either root privileges, i.e. using `sudo`, or
the binary requires the `CAP_NET_RAW` capability;

```bash
# add CAP_NET_RAW to binary to run it as non-root
sudo setcap cap_net_raw+ep unterflow-dump
```

### Usage

```bash
$ unterflow-dump --help
unterflow-dump 0.1.0
Dump unterflow protocol packages

USAGE:
    unterflow-dump [FLAGS] [OPTIONS] --interface <interface>

FLAGS:
    -h, --help       Prints help information
    -y, --pretty     Pretty print message body
    -v               Enable logging, use multiple `v`s to increase verbosity
    -V, --version    Prints version information

OPTIONS:
    -i, --interface <interface>    Interface to capture
    -p, --port <port>...           Ports to capture [default: 51015]
```

### Example

```bash
$ sudo ./target/debug/unterflow-dump -vy
unterflow_dump::network: No interface specified. Selecting first interface found.
unterflow_dump::network: Opening channel for interface: lo
unterflow_dump::network: NetworkInterface { name: "lo", index: 1, mac: Some(00:00:00:00:00:00), ips: [V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), V6(Ipv6Network { addr: ::1, prefix: 128 })], flags: 65609 }
unterflow_dump: Capturing TCP ports: [51015]
==>  Packet: 127.0.0.1:59160 -> 127.0.0.1:51015 (144 bytes; seq: 4002476581)
FrameHeader { length: 129, version: 0, flags: 0, type_id: Message, stream_id: 2 }
TransportHeader { protocol_id: RequestResponse }
RequestResponseHeader { connection_id: 1, request_id: 0 }
MessageHeader { block_length: 11, template_id: 20, schema_id: 0, version: 1 }
ExecuteCommandRequest {
    partition_id: 0,
    key: 18446744073709551615,
    event_type: TaskEvent,
    topic_name: "default-topic",
    command: {"eventType": "CREATE", "retries": 3, "type": "foo", "headers": {"k1": "a", "k2": "b"}, "payload": [129, 167, 112, 97, 121, 108, 111, 97, 100, 123]}, payload (decoded): {"payload": 123}
}

==>  Packet: 127.0.0.1:51015 -> 127.0.0.1:59160 (288 bytes; seq: 539711505)
FrameHeader { length: 269, version: 0, flags: 0, type_id: Message, stream_id: 2 }
TransportHeader { protocol_id: RequestResponse }
RequestResponseHeader { connection_id: 1, request_id: 0 }
MessageHeader { block_length: 10, template_id: 21, schema_id: 0, version: 1 }
ExecuteCommandResponse {
    partition_id: 0,
    key: 4294967544,
    topic_name: "default-topic",
    event: {"eventType": "CREATED", "lockTime": -9223372036854775808, "lockOwner": "", "retries": 3, "type": "foo", "headers": {"bpmnProcessId": "", "workflowDefinitionVersion": -1, "workflowInstanceKey": -1, "activityId": "", "activityInstanceKey": -1, "customHeaders": [], "k1": "a", "k2": "b"}, "payload": [129, 167, 112, 97, 121, 108, 111, 97, 100, 123]}, payload (decoded): {"payload": 123}
}

==>  Packet: 127.0.0.1:59160 -> 127.0.0.1:51015 (16 bytes; seq: 4002476725)
FrameHeader { length: 0, version: 0, flags: 0, type_id: ControlClose, stream_id: 0 }

==>  Packet: 127.0.0.1:51015 -> 127.0.0.1:59160 (16 bytes; seq: 539711793)
FrameHeader { length: 0, version: 0, flags: 0, type_id: ControlEndOfStream, stream_id: 0 }
```

