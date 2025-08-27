use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{collections::HashMap, ops::Deref};
use tokio::{net::UdpSocket, sync::RwLock, task::JoinHandle};

use crate::fm_network::action::FMAction;
use crate::fm_network::client::ClientStatus;
use crate::fm_network::jpeg_decoder::{JPEGDecoder, JPEGHeader};
use crate::fm_network::packet::FMPacket;

const FM_SERVER_PORT: u16 = 3333;
const FM_CLIENT_PORT: u16 = 3334;

lazy_static! {
    static ref CLIENTS: RwLock<HashMap<SocketAddr, ClientStatus>> = RwLock::new(HashMap::new());
    static ref SOCKET_HANDLER: RwLock<SocketHandler> = RwLock::new(SocketHandler {
        socket: None,
        task: None,
        client_live_checker: None,
    });
    static ref LISTENERS: RwLock<Vec<Arc<Listener>>> = RwLock::new(Vec::new());
    static ref JPEG_DECODERS: RwLock<HashMap<SocketAddr, JPEGDecoder>> =
        RwLock::new(HashMap::new());
}

struct Listener {
    callback: Arc<dyn Fn(&FMAction) + Send + Sync>,
}

struct SocketHandler {
    socket: Option<Arc<UdpSocket>>,
    task: Option<JoinHandle<()>>,
    client_live_checker: Option<JoinHandle<()>>,
}

impl SocketHandler {
    pub(crate) fn init(&mut self, socket: UdpSocket) {
        let arc_socket = Arc::new(socket);
        let socket = arc_socket.clone();

        let task = tokio::task::spawn(async move {
            let mut buf = [0; 8192];
            loop {
                match arc_socket.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        let mut clients = CLIENTS.write().await;
                        let origin_len = clients.len();
                        let client = clients
                            .entry(addr)
                            .or_insert_with(|| ClientStatus::new(addr));

                        client.update_heartbeat();
                        let new_len = clients.len();

                        if new_len > origin_len {
                            emit_action(FMAction::ClientChanged {
                                add: Some(addr),
                                remove: None,
                            })
                            .await;
                        }

                        send(addr, FMPacket::Heartbeat).await;

                        dbg!(&len, &addr);

                        let data = &buf[..len];

                        let packet = Arc::new(FMPacket::new(data));
                        let action = FMAction::PacketReceived {
                            addr,
                            packet: packet.clone(),
                        };
                        emit_action(action).await;

                        if let FMPacket::JPEGPacket { header, data } = packet.deref() {
                            decode_jpeg_packet(addr, header.to_owned(), data).await;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                    }
                }
            }
        });

        let live_checker = tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                let mut clients = CLIENTS.write().await;
                let mut pending_remove = Vec::<SocketAddr>::new();

                for (addr, status) in clients.iter() {
                    if !status.is_active() {
                        pending_remove.push(*addr);
                    }
                }

                for addr in pending_remove.iter() {
                    clients.remove(addr);
                    emit_action(FMAction::ClientChanged {
                        add: None,
                        remove: Some(*addr),
                    })
                    .await;
                }
            }
        });

        self.task = Some(task);
        self.socket = Some(socket);
        self.client_live_checker = Some(live_checker);
    }

    fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
        self.socket = None;

        if let Some(live_checker) = self.client_live_checker.take() {
            live_checker.abort();
        }
        self.client_live_checker = None;
    }
}

pub async fn run() {
    let mut handler = SOCKET_HANDLER.write().await;

    if let Some(_) = handler.socket {
        return;
    }

    if let Some(_) = handler.task {
        return;
    }

    let socket_result = UdpSocket::bind(format!("0.0.0.0:{}", FM_SERVER_PORT)).await;
    if let Ok(socket) = socket_result {
        handler.init(socket);
    }
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

pub async fn listen<F>(callback: F)
where
    F: Fn(&FMAction) + Send + Sync + 'static,
{
    let mut writer = LISTENERS.write().await;
    writer.push(Arc::new(Listener {
        callback: Arc::new(callback),
    }));
}

pub async fn send(mut addr: SocketAddr, packet: FMPacket) {
    if let Some(socket) = &SOCKET_HANDLER.read().await.socket {
        if let Some(send_bytes) = packet.to_bytes() {
            addr.set_port(FM_CLIENT_PORT);
            match socket.send_to(send_bytes.as_slice(), addr).await {
                Ok(_) => {
                    dbg!(send_bytes.len(), addr);
                }
                Err(e) => {
                    eprintln!("Error sending data: {}", e);
                }
            }
        }
    }
}

async fn emit_action<'a>(action: FMAction<'a>) {
    for listener in LISTENERS.read().await.iter() {
        listener.callback.deref()(&action);
    }
}

async fn decode_jpeg_packet(addr: SocketAddr, header: JPEGHeader, data: &Vec<u8>) {
    let mut decoders = JPEG_DECODERS.write().await;

    let decoder = decoders
        .entry(addr)
        .or_insert_with(|| JPEGDecoder::new(header));

    match decoder.append_data(header, data) {
        Ok(Some(decoded_data)) => {
            emit_action(FMAction::JpegDecoded {
                addr,
                data: decoded_data,
            })
            .await
        }
        Err(e) => {
            eprintln!("Error appending JPEG data: {}", e);
        }
        _ => {}
    }
}
