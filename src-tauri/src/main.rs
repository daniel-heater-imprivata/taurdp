// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use ironrdp::connector;
use anyhow::Context as _;
use tauri::Manager;

use taurdp::config::Config;
use taurdp::config::Destination;
use taurdp::rdp::{RdpClient, RdpInputEvent};

#[tauri::command]
fn login(server: &str, port: u16, username: &str, password: &str) -> Result<String, String> {
    let response = format!("logging in user {} to {}:{}", username, server, port);

    let log_file = "~/.taurdp.log".to_owned();

    let mut config = Config::parse_args().unwrap(); //.context("CLI arguments parsing")
                                                    // let config = match config {
                                                    //     Some(_) =>
                                                    //     Ok("Success ".to_owned() + &response)
                                                    //     None() =>
                                                    //     Err("Failed ".to_owned() + &response)

    // };

    let destination = Destination {
        name: server.to_string(),
        port,
    };

    let (input_event_sender, input_event_receiver) = RdpInputEvent::create_channel();
    let client = RdpClient {
        config,
        input_event_receiver,
    };
    if port == 3389 {
        Ok("Success ".to_owned() + &response)
    } else {
        Err("Failed ".to_owned() + &response)
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    x: u32,
    y: u32,
}

#[tauri::command]
fn mouse_press(position: &str) {
    println!("mouse pressed at: {}", position);
}

#[tauri::command]
fn keyboard_press(s: String) {
    println!("pressed: {}", s);
}

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            let _id = main_window.listen("event", |event| match event.payload() {
                Some(payload) => mouse_press(payload),
                None => eprintln!("Noise!"),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![login, mouse_press, keyboard_press])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
