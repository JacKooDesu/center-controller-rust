use std::{
    collections::{btree_map::OccupiedEntry, hash_map::Entry, HashMap},
    vec,
};

use lazy_static::lazy_static;
use serde_json::Value;
use tauri::{Emitter, Manager, Runtime, Window};
use tokio::{
    fs::{read_dir, File, ReadDir},
    io::{AsyncRead, AsyncReadExt},
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
        FMAction::HistorySaved(detail) => {
            let _ = window
                .app_handle()
                .emit_to(window.label(), "fm://history_saved", detail);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![])
        .invoke_handler(tauri::generate_handler![
            start_udp,
            stop_udp,
            send_msg,
            query_play_histories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
