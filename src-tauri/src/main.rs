// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Create a second window at runtime
            // tauri::WindowBuilder::new(app, "second", tauri::WindowUrl::App("/second".into()))
            //     .inner_size(540.0, 480.0)
            //     .title("tauri-frontend-bug Window 2")
            //     .center()
            //     .build()
            //     .expect("couldn't create a second window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
