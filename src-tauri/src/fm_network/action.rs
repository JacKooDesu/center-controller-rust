use std::{net::SocketAddr, sync::Arc};

use serde::Serialize;

use crate::fm_network::packet::FMPacket;

pub enum FMAction<'a> {
    ClientChanged(ClientChangedDetail),
    JpegDecoded(JpegDecodedDetail<'a>),
    PacketReceived {
        addr: SocketAddr,
        packet: Arc<FMPacket>,
    },
}

#[derive(Serialize, Debug)]
pub(crate) struct ClientChangedDetail {
    add: Option<SocketAddr>,
    remove: Option<SocketAddr>,
}

#[derive(Serialize, Debug)]
pub(crate) struct JpegDecodedDetail<'a> {
    addr: SocketAddr,
    data: &'a Vec<u8>,
}

impl ClientChangedDetail {
    fn new(add: Option<SocketAddr>, remove: Option<SocketAddr>) -> Self {
        Self { add, remove }
    }
    pub fn added(addr: SocketAddr) -> Self {
        Self::new(Some(addr), None)
    }

    pub fn removed(addr: SocketAddr) -> Self {
        Self::new(None, Some(addr))
    }
}

impl<'a> JpegDecodedDetail<'a> {
    pub fn new(addr: SocketAddr, data: &'a Vec<u8>) -> Self {
        Self { addr, data }
    }
}
