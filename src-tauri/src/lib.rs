use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri_plugin_log::{Target, TargetKind};
mod app;
use app::App;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_admob::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            app.manage(App::new(app_handle));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
