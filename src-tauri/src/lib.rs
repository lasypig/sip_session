pub mod commands;
pub mod sip;

use tauri::Manager;

pub struct AppState {
    pub session_manager: std::sync::Mutex<Option<sip::session::SessionManager>>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            session_manager: std::sync::Mutex::new(None),
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_pcap_file,
            commands::get_sessions,
            commands::get_message_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
