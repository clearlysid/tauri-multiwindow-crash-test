// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
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
