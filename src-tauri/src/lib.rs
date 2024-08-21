// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_selection;

use arboard::Clipboard;
use futures::future::join_all;
use ipnet::Ipv4Net;
use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tauri::{
    include_image,
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_global_shortcut::Shortcut;
use tokio::net::TcpStream;

async fn scan_port(target: Ipv4Addr, port: u16, timeout: u64) -> (Ipv4Addr, bool) {
    let timeout = tokio::time::Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(IpAddr::V4(target), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => (target, true),
        _ => (target, false),
    }
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
