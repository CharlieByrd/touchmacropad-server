mod commands;
mod server;
use std::thread;
use commands::macros_command;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    thread::spawn(|| {
        server::mdns_server::start_mdns_server();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![macros_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
