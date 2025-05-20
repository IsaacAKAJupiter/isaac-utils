// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_selection;

use arboard::Clipboard;
use futures::future::join_all;
use futures::{SinkExt, StreamExt};
use ipnet::Ipv4Net;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tauri::Listener;
use tauri::{
    include_image,
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::Shortcut;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

async fn scan_port(target: Ipv4Addr, port: u16, timeout: u64) -> (Ipv4Addr, bool) {
    let timeout = tokio::time::Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(IpAddr::V4(target), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => (target, true),
        _ => (target, false),
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream, app: &AppHandle) {
    if let Err(e) = handle_connection(peer, stream, app).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream, app: &AppHandle) -> Result<()> {
    let ws_stream: tokio_tungstenite::WebSocketStream<TcpStream> =
        accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);
    let (ws_sender, mut ws_receiver) = ws_stream.split();
    let ws_sender_arc = Arc::new(Mutex::new(ws_sender));

    // State variables.
    let available_states: [&'static str; 2] = ["file", "text"];
    let mut state: String = "".to_string();

    // File variables.
    let mut file_id: String = "".to_string();
    let mut file_size: i64 = -1;
    let mut file_processed: i64 = 0;
    let file_save_path_arc = Arc::new(Mutex::new("".to_string()));

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_close() {
            break;
        }

        if msg.is_ping() {
            let mut sender_lock = ws_sender_arc.lock().await;
            let _ = sender_lock.send(Message::Pong(msg.into_data())).await;
            continue;
        }

        let msg_text = if msg.is_text() { msg.to_string() } else { "".to_string() };

        // If no state and not a text message, ignore it.
        if state == "" && !msg.is_text() {
            continue;
        }

        // If no state, make sure the message is a string and a valid state.
        if state == "" && msg.is_text() {
            let msg_text_str = msg_text.as_str();
            let state_index = available_states.iter().position(|&r| r == msg_text_str);
            if state_index.is_some() {
                state = msg_text;
            }
            continue;
        }

        // If file state, look for file name/size.
        if state == "file" && msg.is_text() && file_size == -1 {
            let msg_split: Vec<_> = msg_text.split("<|>").collect();
            if msg_split.len() != 3 {
                println!("Invalid msg sent: {}", msg_text);
                continue;
            }

            // Parse the file size.
            let parsed = msg_split[2].trim().parse::<i64>();
            if parsed.is_err() {
                println!("Invalid file size sent with message: {}", msg_text);
                continue;
            }

            // Set the name and size.
            file_id = msg_split[0].to_string();
            file_size = parsed.unwrap();
            file_processed = 0;

            if let Some(window) = app.get_webview_window("main") {
                // Ask if we want the file.
                let _ = window.emit("e_p2p", json!({
                    "event": "ask_file",
                    "data": {
                        "id": file_id,
                        "file_name": msg_split[1].to_string(),
                        "file_size": file_size,
                        "peer": peer
                    }
                }));

                // Wait for the file response.
                let ws_sender_clone = Arc::clone(&ws_sender_arc);
                let file_save_path_clone = Arc::clone(&file_save_path_arc);
                window.once(format!("e_p2p_ask_file_{}", file_id), move |event| {
                    let mut message_to_send: String = "0".to_string();
                    let mut path_to_change: String = "".to_string();
                    let payload_str = event.payload();
                    let payload: Vec<_> = payload_str.trim_matches('"').split("<|>").collect();
                    if payload.len() == 2 && payload[0] == "1" {
                        message_to_send = "1".to_string();
                        path_to_change = payload[1].to_string();
                    }

                    tokio::spawn(async move {
                        if !path_to_change.is_empty() {
                            let mut path_guard = file_save_path_clone.lock().await;
                            *path_guard = path_to_change;
                        }

                        let mut sender_lock = ws_sender_clone.lock().await;
                        if let Err(e) = sender_lock.send(Message::Text(Utf8Bytes::from(message_to_send))).await {
                            eprintln!("Error sending WebSocket message: {}", e);
                        }
                    });
                });
            }

            continue;
        }

        // If in a file state and looking for data.
        if state == "file" && file_size != -1 {
            // If not binary, continue.
            if !msg.is_binary() {
                continue;
            }

            let data = msg.into_data();
            let data_len = data.len();
            let is_start = file_processed < 1;
            file_processed += data_len.try_into().unwrap_or(0);

            // Write the data.
            let path_guard = file_save_path_arc.lock().await;
            let file = if is_start { OpenOptions::new().create(true).write(true).truncate(true).open(&*path_guard) } else { OpenOptions::new().create(true).append(true).open(&*path_guard) };
            if file.is_ok() {
                let mut buf = std::io::BufWriter::new(file.unwrap());
                let result = buf.write_all(&data);
                buf.flush()?;

                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("e_p2p", json!({
                        "event": "file_data",
                        "data": {
                            "id": file_id,
                            "chunk_size": data_len,
                            "total_processed": file_processed,
                            "successful_write": result.is_ok()
                        }
                    }));
                }

                if file_processed >= file_size {
                    println!("Finished processing file!");
                    state = "".to_string();
                    file_size = -1;
                    file_processed = 0;
                    let mut sender_lock = ws_sender_arc.lock().await;
                    let _ = sender_lock.close().await;
                    continue;
                }
            } else {
                println!("{:?}", file.unwrap_err());
            }

            continue;
        }

        // Handle normal text data.
        if state == "text" && msg.is_text() {
            // Send the data to the main window.
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("e_p2p", json!({
                    "event": "text_received",
                    "data": {
                        "text": msg_text,
                        "peer": peer
                    }
                }));
            }

            state = "".to_string();
            continue;
        }
    }

    Ok(())
}

#[tauri::command]
fn c_unix_to_readable(config: serde_json::Map<String, serde_json::Value>, app: AppHandle) {
    // Get the selected content.
    let content = get_selection::get_text();

    // Handle the content from the clipboard.
    let parsed = content.trim().parse::<f64>();
    match parsed {
        Ok(f) => {
            // Show the unix popup window.
            if let Some(window) = app.get_webview_window("unix_popup") {
                let _ = window.emit(
                    "e_unix_popup",
                    json!({"number": f as i64, "config": config}),
                );
            }
        }
        Err(_) => {}
    };
}

#[tauri::command]
fn c_copy(value: String) -> bool {
    let clipboard = Clipboard::new();
    match clipboard {
        Ok(mut c) => c.set_text(value).is_ok(),
        Err(_) => false,
    }
}

#[tauri::command]
fn c_valid_shortcut(shortcut: String) -> bool {
    let result = Shortcut::try_from(shortcut.as_str());
    result.is_ok()
}

#[tauri::command]
async fn c_check_ports() -> serde_json::Value {
    match netdev::get_default_interface() {
        Ok(interface) => {
            if interface.ipv4.is_empty() {
                return json!({"results": null});
            }

            println!("Default Interface:");
            println!("\tIPv4: {:?}", interface.ipv4);
            println!("\tIP: {:?}", interface.ipv4[0].addr);
            println!("\tSubnet Mask: {:?}", interface.ipv4[0].netmask);
            println!("\tPrefix Len: {:?}", interface.ipv4[0].prefix_len);

            match Ipv4Net::new(interface.ipv4[0].addr, interface.ipv4[0].prefix_len) {
                Ok(nw) => {
                    let results = join_all(nw.hosts().map(|host| scan_port(host, 8888, 1))).await;
                    return json!({"results": results});
                }
                Err(e) => {
                    println!("NW Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    json!({"results": null})
}

fn make_tray(app: &tauri::App) -> Result<(), tauri::Error> {
    let show_hide = MenuItemBuilder::with_id("show_hide", "Show/Hide").build(app)?;
    let divider = PredefinedMenuItem::separator(app)?;
    let check_for_update =
        MenuItemBuilder::with_id("check_for_update", "Check for Update").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&show_hide, &divider, &check_for_update, &quit])
        .build()?;
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show_hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().is_ok_and(|x| x) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.center();
                    }
                }
            }
            "check_for_update" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("e_check_for_update", json!({}));
                }
            }
            "quit" => {
                app.cleanup_before_exit();
                std::process::exit(0);
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                show_main_window(app);
            }
        })
        .tooltip("Isaac Utils")
        .icon(include_image!("./icons/icon.png"))
        .build(app)?;

    Ok(())
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_main_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            c_unix_to_readable,
            c_copy,
            c_valid_shortcut,
            c_check_ports,
        ])
        .setup(|app| {
            let _ = make_tray(&app);

            let handler_clone = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let handler_clone = handler_clone.to_owned();
                let addr = "0.0.0.0:15446";
                let listener = TcpListener::bind(&addr).await.expect("Can't listen.");
                println!("Listening on: {}", addr);

                while let Ok((stream, _)) = listener.accept().await {
                    let peer = stream
                        .peer_addr()
                        .expect("connected streams should have a peer address");
                    println!("Peer address: {}", peer);

                    let _ = accept_connection(peer, stream, &handler_clone).await;
                }
            });

            // Uncomment below to automatically open devtools for the unix popup window.
            // #[cfg(debug_assertions)]
            // {
            //     if let Some(window) = app.get_webview_window("unix_popup") {
            //         window.open_devtools();
            //     }
            // }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Focused(focused) => {
                // If not the main window and not focused, hide it.
                if window.label() == "unix_popup" && !focused {
                    let _ = window.hide();
                }
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // If the window is attempting to close, prevent and just hide.
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
            _ => {}
        });
}
