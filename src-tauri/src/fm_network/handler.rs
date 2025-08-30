use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::{net::UdpSocket, task::JoinHandle};

use crate::fm_network::action::{
    ClientChangedDetail, FMAction, HistorySavedDetail, JpegDecodedDetail,
};
use crate::fm_network::client::ClientStatus;
use crate::fm_network::jpeg_decoder::{JPEGDecoder, JPEGHeader};
use crate::fm_network::packet::FMPacket;
use crate::fm_network::{
    emit_action, send, CLIENTS, FM_CLIENT_PORT, FM_SERVER_PORT, JPEG_DECODERS, PLAY_HISTORY_PATH,
};

pub(crate) struct SocketHandler {
    socket: Option<Arc<UdpSocket>>,
    task: Option<JoinHandle<()>>,
    client_live_checker: Option<JoinHandle<()>>,
}

impl SocketHandler {
    pub fn new() -> Self {
        Self {
            socket: None,
            task: None,
            client_live_checker: None,
        }
    }

    pub(crate) async fn run(&mut self) -> bool {
        if let Some(_) = self.socket {
            return false;
        }

        if let Some(_) = self.task {
            return false;
        }

        let socket_result = UdpSocket::bind(format!("0.0.0.0:{}", FM_SERVER_PORT)).await;
        if let Ok(socket) = socket_result {
            self.init(socket);
            return true;
        }

        return false;
    }

    fn init(&mut self, socket: UdpSocket) {
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
                            emit_action(FMAction::ClientChanged(ClientChangedDetail::added(addr)))
                                .await;
                        }

                        send(addr.into(), FMPacket::Heartbeat).await;

                        let data = &buf[..len];

                        let packet = Arc::new(FMPacket::new(data));
                        let action = FMAction::PacketReceived {
                            addr,
                            packet: packet.clone(),
                        };
                        emit_action(action).await;

                        match packet.deref() {
                            FMPacket::JPEGPacket { header, data } => {
                                decode_jpeg_packet(addr, header.to_owned(), data).await
                            }
                            FMPacket::PlayHistoryPacket { json } => {
                                save_play_history(&json).await;
                            }
                            _ => {}
                        };
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

                if !pending_remove.is_empty() {
                    dbg!(&pending_remove);
                }

                for addr in pending_remove.iter() {
                    clients.remove(addr);
                    emit_action(FMAction::ClientChanged(ClientChangedDetail::removed(*addr))).await;
                }
            }
        });

        self.task = Some(task);
        self.socket = Some(socket);
        self.client_live_checker = Some(live_checker);

        println!("SocketHandler initialized at {:?}", self.socket);
    }

    pub(crate) fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
        self.socket = None;

        if let Some(live_checker) = self.client_live_checker.take() {
            live_checker.abort();
        }
        self.client_live_checker = None;

        println!("SocketHandler stopped");
    }

    pub(crate) async fn send(&self, mut addr: SocketAddr, packet: FMPacket) {
        if let Some(socket) = &self.socket {
            if let Some(send_bytes) = packet.to_bytes() {
                addr.set_port(FM_CLIENT_PORT);
                match socket.send_to(send_bytes.as_slice(), addr).await {
                    Ok(_) => {
                        if let FMPacket::StringPacket { data } = packet {
                            dbg!(&data, addr);
                        };
                    }
                    Err(e) => {
                        eprintln!("Error sending data: {}", e);
                    }
                }
            }
        }
    }
}

async fn decode_jpeg_packet(addr: SocketAddr, header: JPEGHeader, data: &Vec<u8>) {
    let mut decoders = JPEG_DECODERS.write().await;

    let decoder = decoders
        .entry(addr)
        .or_insert_with(|| JPEGDecoder::new(header));

    match decoder.append_data(header, data) {
        Ok(Some(decoded_data)) => {
            emit_action(FMAction::JpegDecoded(JpegDecodedDetail::new(
                addr,
                decoded_data,
            )))
            .await
        }
        Err(e) => {
            eprintln!("Error appending JPEG data: {}", e);
        }
        _ => {}
    }
}

async fn save_play_history(json: &str) {
    if let Ok(play_history_map) = serde_json::from_str::<HashMap<String, Value>>(json) {
        if let Some(user_id) = play_history_map.get("userId") {
            let user_id = user_id.as_str().unwrap_or_else(|| "unknown");
            let mut file_path = PathBuf::new();
            file_path.push(PLAY_HISTORY_PATH);
            tokio::fs::create_dir_all(&file_path).await.ok();
            file_path.push(format!("{}.json", user_id));

            if let Ok(mut file) = File::create(&file_path).await {
                if let Err(e) = file.write_all(json.as_bytes()).await {
                    eprintln!("Error writing play history to file: {}", e);
                } else {
                    emit_action(FMAction::HistorySaved(HistorySavedDetail::new(
                        user_id,
                        file_path.to_str().unwrap_or("<<path missing>>"),
                    )))
                    .await;
                    println!("Play history saved to file: {}", file_path.display());
                }
            } else {
                eprintln!("Error creating play history file: {}", file_path.display());
            }
        }
    }
}
