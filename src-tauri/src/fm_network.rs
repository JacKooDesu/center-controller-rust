pub mod action;
pub mod client;
pub mod handler;
pub mod jpeg_decoder;
pub mod packet;

use std::{collections::HashMap, net::SocketAddr, ops::Deref, sync::Arc};

use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::fm_network::{
    action::FMAction, client::ClientStatus, handler::SocketHandler, jpeg_decoder::JPEGDecoder,
    packet::FMPacket,
};

const FM_SERVER_PORT: u16 = 3333;
const FM_CLIENT_PORT: u16 = 3334;

lazy_static! {
    static ref CLIENTS: RwLock<HashMap<SocketAddr, ClientStatus>> = RwLock::new(HashMap::new());
    static ref SOCKET_HANDLER: RwLock<SocketHandler> = RwLock::new(SocketHandler::new());
    static ref LISTENERS: RwLock<Vec<Arc<Listener>>> = RwLock::new(Vec::new());
    static ref JPEG_DECODERS: RwLock<HashMap<SocketAddr, JPEGDecoder>> =
        RwLock::new(HashMap::new());
}

struct Listener {
    callback: Arc<dyn Fn(&FMAction) + Send + Sync>,
}

pub async fn listen<F>(callback: F)
where
    F: Fn(&FMAction) + Send + Sync + 'static,
{
    let mut writer = LISTENERS.write().await;
    writer.push(Arc::new(Listener {
        callback: Arc::new(callback),
    }));
}

pub async fn run() -> bool {
    let mut handler = SOCKET_HANDLER.write().await;

    handler.run().await
}

pub async fn stop() {
    let mut handler = SOCKET_HANDLER.write().await;
    handler.stop();

    let mut clients = CLIENTS.write().await;
    clients.clear();

    let mut listeners = LISTENERS.write().await;
    listeners.clear();

    let mut decoders = JPEG_DECODERS.write().await;
    decoders.clear();
}

pub enum Addr {
    String(String),
    SocketAddr(SocketAddr),
}

impl Into<SocketAddr> for Addr {
    fn into(self) -> SocketAddr {
        match self {
            Addr::String(ip) => {
                let addr: SocketAddr = format!("{}:{}", ip, FM_CLIENT_PORT)
                    .parse()
                    .expect("Invalid IP address");
                addr
            }
            Addr::SocketAddr(addr) => addr,
        }
    }
}

impl From<String> for Addr {
    fn from(ip: String) -> Self {
        Addr::String(ip)
    }
}

impl From<SocketAddr> for Addr {
    fn from(addr: SocketAddr) -> Self {
        Addr::SocketAddr(addr)
    }
}

pub async fn send(addr: Addr, packet: FMPacket) {
    SOCKET_HANDLER.read().await.send(addr.into(), packet).await;
}

pub(crate) async fn emit_action<'a>(action: FMAction<'a>) {
    for listener in LISTENERS.read().await.iter() {
        listener.callback.deref()(&action);
    }
}
