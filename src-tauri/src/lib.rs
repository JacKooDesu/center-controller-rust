use tauri::{Emitter, Manager, Runtime, Window};

use crate::fm_network::{action::FMAction, packet::FMPacket};

mod fm_network;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_udp() {
    fm_network::handler::run().await;
}

#[tauri::command]
async fn stop_udp() {
    fm_network::handler::stop().await;
}

#[tauri::command]
async fn add_jpg_decoded_listener<R: Runtime>(window: Window<R>) {
    fm_network::handler::listen(move |data| {
        if let FMAction::JpegDecoded { addr, data } = data {
            let _ = window
                .app_handle()
                .emit_to(window.label(), "fm://jpeg_decoded", data);
        }
    })
    .await;
}

#[tauri::command]
async fn add_client_changed_listener<R: Runtime>(window: Window<R>) {
    fm_network::handler::listen(move |data| {
        if let FMAction::ClientChanged { add, remove } = data {
            let _ = window
                .app_handle()
                .emit_to(window.label(), "fm://client_changed", (add, remove));
            dbg!(
                "Emitted Client Changed - Added: {:?}, Removed: {:?}",
                add,
                remove
            );
        }
    })
    .await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![])
        .invoke_handler(tauri::generate_handler![
            greet,
            start_udp,
            stop_udp,
            add_jpg_decoded_listener,
            add_client_changed_listener
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
