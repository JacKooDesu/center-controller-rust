use tauri::{Emitter, Manager, Runtime, Window};

use crate::fm_network::action::FMAction;

mod fm_network;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_udp<R: Runtime>(window: Window<R>) {
    if !fm_network::run().await {
        return;
    }

    fm_network::listen(move |data| match data {
        FMAction::ClientChanged(detail) => {
            let _ =
                window
                    .app_handle()
                    .emit_to(window.label(), "fm://client_changed", detail);
            dbg!(detail);
        }
        FMAction::JpegDecoded(detail) => {
            let _ = window
                .app_handle()
                .emit_to(window.label(), "fm://jpeg_decoded", detail);
        }
        _ => {}
    })
    .await;
}

#[tauri::command]
async fn stop_udp() {
    fm_network::stop().await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![])
        .invoke_handler(tauri::generate_handler![greet, start_udp, stop_udp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
