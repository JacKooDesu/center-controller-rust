use std::{net::SocketAddr, sync::Arc};

use crate::fm_network::packet::FMPacket;

pub enum FMAction<'a> {
    ClientChanged {
        add: Option<SocketAddr>,
        remove: Option<SocketAddr>,
    },
    JpegDecoded {
        addr: SocketAddr,
        data: &'a Vec<u8>,
    },
    PacketReceived {
        addr: SocketAddr,
        packet: Arc<FMPacket>,
    },
}
