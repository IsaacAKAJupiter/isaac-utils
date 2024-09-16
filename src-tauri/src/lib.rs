// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_selection;

use arboard::Clipboard;
use futures::future::join_all;
use futures::{SinkExt, StreamExt};
use ipnet::Ipv4Net;
use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tauri::Listener;
use tauri::{
    include_image,
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::Shortcut;
use tokio::net::{TcpListener, TcpStream};
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
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));

    // State variables.
    let available_states = ["file", "text"];
    let mut state = "".to_string();

    // File variables.
    let mut file_name = "".to_string();
    let mut file_size = -1 as i64;

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_close() {
                            break;
                        }

                        let msg_text = if msg.is_text() {msg.to_string()} else {"".to_string()};

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
                            if msg_split.len() != 2 {
                                println!("Invalid msg sent: {}", msg_text);
                                continue;
                            }

                            // Parse the file size.
                            let parsed = msg_split[1].trim().parse::<i64>();
                            if parsed.is_err() {
                                println!("Invalid file size sent with message: {}", msg_text);
                                continue;
                            }

                            // Set the name and size.
                            file_name = msg_split[0].to_string();
                            file_size = parsed.unwrap();

                            if let Some(window) = app.get_webview_window("main") {
                                // Ask if we want the file.
                                let _ = window.emit("e_p2p", json!({
                                    "event": "ask_file",
                                    "data": {
                                        "file_name": file_name,
                                        "file_size": file_size,
                                        "peer": peer
                                    }
                                }));

                                // Wait for the file response.
                                window.once("e_p2p_ask_file", |event| {
                                    // If not allowed.
                                    // if !event.data {
                                    //     file_name = "".to_string();
                                    //     file_size = -1;
                                    //     return
                                    // }

                                    println!("{}", event.payload());

                                    // If allowed, send the event to the peer that we are good to send.
                                    // ws_sender.send(Message::Text("1".to_owned())).await?;
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

                            // TODO: Handle binary data.
                        }

                        // Send the data to the main window.
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("e_random_message", json!({
                                "msg": msg.into_text().unwrap_or("could not convert message into text".to_string())
                            }));
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
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
            c_check_ports
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
