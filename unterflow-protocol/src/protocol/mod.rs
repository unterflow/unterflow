macro_rules! align {
    ($value:expr, $alignment:expr) => { ($value + ($alignment - 1)) & !($alignment - 1) }
}

pub mod client;
pub mod gossip;
pub mod log;
pub mod management;
pub mod raft;
pub mod transport;