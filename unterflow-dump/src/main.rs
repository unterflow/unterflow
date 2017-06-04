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

    loggerv::init_with_verbosity(args.occurrences_of("v"))?;

    let interface = args.value_of("interface").expect("Interface required");
    let port = value_t!(args, "port", u16)?;

    let (_, mut rx) = network::channel_for_interface(interface)?;


    let mut last = None;
    let mut iter = rx.iter();

    let same = |last: &Option<CapturedPacket>, packet: &CapturedPacket| match *last {
        Some(ref last) => last == packet,
        _ => false,
    };

    loop {
        if let Ok(packet) = iter.next() {
            if let Some(packet) = network::capture_packet(&packet) {
                if !same(&last, &packet) && packet.len() > 0 && packet.has_port(port) {
                    if let Err(e) = protocol::dump_packet(&packet) {
                        error!("Unable to parse packet {:?}: {}", packet, e);
                    }
                    last = Some(packet);
                }
            }
        }
    }

}
