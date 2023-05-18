// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Context as _;
use ironrdp::connector;
use ironrdp::pdu::gcc::KeyboardType;
use ironrdp::pdu::nego::SecurityProtocol;
use ironrdp::pdu::rdp::capability_sets::MajorPlatformType;
use tauri::Manager;
use whoami;

use taurdp::config::Destination;
use taurdp::rdp::{RdpClient, RdpInputEvent};
use tokio::runtime;

#[tauri::command]
fn login(server: &str, port: u16, username: &str, password: &str) -> Result<String, String> {
    let response = format!("logging in user {} to {}:{}", username, server, port);

    let log_file = "~/.taurdp.log".to_owned();

    let connector = connector::Config {
        username: username.to_string(),
        password: password.to_string(),
        domain: None,
        security_protocol: SecurityProtocol::HYBRID,
        keyboard_type: KeyboardType::IbmEnhanced,
        keyboard_subtype: 0,
        keyboard_functional_keys_count: 12,
        ime_file_name: String::from(""),
        dig_product_id: "TauRDP".to_string(),
        desktop_size: connector::DesktopSize {
            width: 800,
            height: 600,
        },
        graphics: None,
        client_build: 0,
        bitmap: None,
        client_name: whoami::hostname(),
        // NOTE: hardcode this value like in freerdp
        // https://github.com/FreeRDP/FreeRDP/blob/4e24b966c86fdf494a782f0dfcfc43a057a2ea60/libfreerdp/core/settings.c#LL49C34-L49C70
        client_dir: "C:\\Windows\\System32\\mstscax.dll".to_owned(),
        platform: match whoami::platform() {
            whoami::Platform::Windows => MajorPlatformType::Windows,
            whoami::Platform::Linux => MajorPlatformType::Unix,
            whoami::Platform::MacOS => MajorPlatformType::Macintosh,
            whoami::Platform::Ios => MajorPlatformType::IOs,
            whoami::Platform::Android => MajorPlatformType::Android,
            _ => MajorPlatformType::Unspecified,
        },
    };

    let destination = Destination {
        name: server.to_string(),
        port,
    };

    let tauri_config = taurdp::config::Config {
        log_file,
        destination,
        connector,
    };

    let (input_event_sender, input_event_receiver) = RdpInputEvent::create_channel();
    let client = RdpClient {
        config: tauri_config,
        input_event_receiver,
    };

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("unable to create tokio runtime").unwrap();

    //debug!("Start RDP thread");
    std::thread::spawn(move || {
        rt.block_on(client.run());
    });

    //debug!("Run GUI");
    //gui.run(input_event_sender);

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
