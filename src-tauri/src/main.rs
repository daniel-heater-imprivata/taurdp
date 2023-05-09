// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn login(server: &str, port: u16, username: &str, _password: &str) -> Result<String, String> {
    let response = format!("logging in user {} to {}:{}", username, server, port);

    if port == 3389 {
        Ok("Success ".to_owned() + &response)
    } else {
        Err("Failed ".to_owned() + &response)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
