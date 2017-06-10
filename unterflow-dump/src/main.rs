#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate loggerv;

extern crate pnet;
extern crate unterflow_protocol;

mod cli;
mod errors;
mod network;
mod protocol;

use errors::*;
use network::CapturedPacket;

fn main() {
    if let Err(error) = try_main() {
        error!("{}", error);

        for error in error.iter().skip(1) {
            error!("Caused by: {}", error);
        }

        if let Some(backtrace) = error.backtrace() {
            error!("Backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = cli::app().get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("v") + 1)?;

    if args.is_present("list-interfaces") {
        info!("Listing network interfaces");
        network::list_interfaces();
        return Ok(());
    }

    let interface = args.value_of("interface");
    let (_, mut rx) = network::channel_for_interface(interface)?;

    let ports = values_t!(args, "port", u16)?;
    info!("Capturing TCP ports: {:?}", ports);

    let pretty = args.is_present("pretty");

    let mut last = None;
    let mut iter = rx.iter();

    let same = |last: &Option<CapturedPacket>, packet: &CapturedPacket| match *last {
        Some(ref last) => last == packet,
        _ => false,
    };

    loop {
        if let Ok(packet) = iter.next() {
            if let Some(packet) = network::capture_packet(&packet) {
                if !same(&last, &packet) && packet.len() > 0 && packet.has_port(&ports) {
                    let mut read_bytes = 0;
                    while read_bytes < packet.len() {
                        let mut payload = packet.payload();
                        match protocol::Protocol::parse(&mut payload, read_bytes as u64, pretty) {
                            Ok(protocol) => {
                                match protocol.frame {
                                    Some(ref frame) => {
                                        read_bytes += frame.message_length();
                                        println!("==>  Packet: {}", packet);
                                        println!("{}", protocol);
                                    }
                                    None => {
                                        warn!("Expected more bytes in packet: {} < {}. Skipping remaining bytes.",
                                              read_bytes,
                                              packet.len());
                                        break;
                                    }
                                }
                            }
                            Err(e) => error!("Unable to parse packet {:?}: {}", packet, e),
                        }
                    }

                    last = Some(packet);
                }
            }
        }
    }

}
