#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, something_else: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from Rust! Now go do {}.",
        name, something_else
    )
}

#[tauri::command]
fn do_something(a_param: &str) -> String {
    format!("Echo: {}", a_param)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, do_something])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
