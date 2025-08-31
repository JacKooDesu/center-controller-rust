use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

use lazy_static::lazy_static;
use serde_json::Value;
use tauri::{Emitter, Manager, Runtime, Window};
use tokio::{
    fs::{read_dir, File},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::RwLock,
};

use crate::fm_network::{action::FMAction, packet::FMPacket, PLAY_HISTORY_PATH};

mod fm_network;

lazy_static! {
    static ref PLAY_HISTORY_CACHE: RwLock<HashMap<String, HashMap<String, Value>>> =
        RwLock::new(HashMap::new());
}

#[tauri::command]
async fn start_udp<R: Runtime>(window: Window<R>) {
    if !fm_network::run().await {
        return;
    }

    fm_network::listen(move |data| match data {
        FMAction::ClientChanged(detail) => {
            let _ = window
                .app_handle()
                .emit_to(window.label(), "fm://client_changed", detail);
            dbg!(detail);
        }
        FMAction::JpegDecoded(detail) => {
            let _ = window
                .app_handle()
                .emit_to(window.label(), "fm://jpeg_decoded", detail);
        }
        FMAction::HistoryReceived(detail) => {
            let map = detail.map.to_owned();
            let id = detail.player_id.to_owned();
            let value = window.clone();
            let _ = tokio::task::spawn(async move {
                if let Some(path) = save_play_history(&id, &map).await {
                    let _ = value.app_handle().emit_to(
                        value.label(),
                        "fm://history_saved",
                        (&id, &path),
                    );

                    let mut cache = PLAY_HISTORY_CACHE.write().await;
                    cache.insert(id, map);
                };
            });
        }
        _ => {}
    })
    .await;
}

#[tauri::command]
async fn stop_udp() {
    fm_network::stop().await;
}

#[tauri::command]
async fn send_msg(addr: String, msg: String) {
    dbg!(&addr, &msg);
    fm_network::send(addr.into(), FMPacket::StringPacket { data: msg }).await;
}

#[tauri::command]
async fn query_play_histories() -> Result<String, String> {
    let mut category = HashMap::<String, String>::new();

    if let Ok(mut r) = read_dir(PLAY_HISTORY_PATH).await {
        let mut cache = PLAY_HISTORY_CACHE.write().await;
        let mut content = String::new();

        while let Ok(Some(dir_entry)) = r.next_entry().await {
            if let Some(path) = dir_entry.path().to_str() {
                match cache.entry(path.into()) {
                    Entry::Vacant(entry) => {
                        if let Ok(mut file) = File::open(path).await {
                            if let Ok(_) = file.read_to_string(&mut content).await {
                                if let Ok(map) =
                                    serde_json::from_str::<HashMap<String, Value>>(&content)
                                {
                                    if let Some(id) =
                                        map.get("userId").map(|v| v.as_str()).flatten()
                                    {
                                        category.insert(id.into(), path.into());
                                        entry.insert(map);
                                    }
                                }
                            }
                        }
                    }
                    Entry::Occupied(entry) => {
                        let v = entry.get();
                        let k = entry.key();
                        if let Some(id) = v.get("userId").map(|v| v.as_str()).flatten() {
                            category.insert(id.into(), k.into());
                        }
                    }
                }
            }

            content.clear();
        }
    }

    if let Ok(string) = serde_json::ser::to_string(&category) {
        Ok(string)
    } else {
        Err("Failed query play history".into())
    }
}

#[tauri::command]
async fn get_history(key: String) -> Result<HashMap<String, Value>, ()> {
    let cache = PLAY_HISTORY_CACHE.read().await;

    if let Some(data) = cache.get(&key) {
        return Ok(data.to_owned());
    }

    Err(())
}

async fn save_play_history(user_id: &String, map: &HashMap<String, Value>) -> Option<String> {
    let mut file_path = PathBuf::new();
    file_path.push(PLAY_HISTORY_PATH);
    tokio::fs::create_dir_all(&file_path).await.ok();
    file_path.push(format!("{}.json", user_id));

    if let Ok(json) = serde_json::to_string(&map) {
        if let Ok(mut file) = File::create(&file_path).await {
            if let Err(e) = file.write_all(json.as_bytes()).await {
                eprintln!("Error writing play history to file: {}", e);
            } else if let Some(path) = file_path.to_str() {
                println!("Play history saved to file: {}", file_path.display());

                return Some(path.into());
            }
        } else {
            eprintln!("Error creating play history file: {}", file_path.display());
        }
    }

    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![])
        .invoke_handler(tauri::generate_handler![
            start_udp,
            stop_udp,
            send_msg,
            query_play_histories,
            get_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
