use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::Listener;
use tauri::{AppHandle, Emitter, Manager};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

pub async fn start(handler_clone: AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "0.0.0.0:15446";

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("Could not get peer address for incoming connection: {}", e);
                continue;
            }
        };

        println!("Peer address: {}", peer);

        let app_clone_for_task = handler_clone.clone();
        let handle = tokio::spawn(async move {
            accept_connection(peer, stream, &app_clone_for_task).await
        });

        if let Err(join_error) = handle.await {
            if join_error.is_panic() {
                eprintln!("A WebSocket connection handler panicked: {:?}", join_error);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("WebSocket handler panicked: {:?}", join_error.into_panic()),
                )));
            } else {
                eprintln!("WebSocket connection handler task failed: {}", join_error);
            }
        }
    }

    Ok(())
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream, app: &AppHandle) {
    if let Err(e) = handle_connection(peer, stream, app).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => {
                println!("Connection for {} closed gracefully or protocol error: {:?}", peer, e);
            },
            err => println!("Error processing connection: {}", err),
        }
    }

    println!("accept_connection task for {} completed.", peer);
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream, app: &AppHandle) -> Result<()> {
    let ws_stream: tokio_tungstenite::WebSocketStream<TcpStream> = match accept_async(stream).await {
        Ok(s) => s,
        Err(e) => {
            println!("WebSocket handshake failed for {}: {:?}", peer, e);
            return Err(e);
        }
    };

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
    let mut file_handle: Option<tokio::fs::File> = None;

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_close() {
            println!("Closing!");
            break;
        }

        if msg.is_ping() {
            let mut sender_lock = ws_sender_arc.lock().await;
            let _ = sender_lock.send(Message::Pong(msg.into_data())).await;
            continue;
        }

        let msg_text = if msg.is_text() {
            msg.to_string()
        } else {
            "".to_string()
        };

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
                let _ = window.emit(
                    "e_p2p",
                    json!({
                        "event": "ask_file",
                        "data": {
                            "id": file_id,
                            "file_name": msg_split[1].to_string(),
                            "file_size": file_size,
                            "peer": peer
                        }
                    }),
                );

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
                        if let Err(e) = sender_lock
                            .send(Message::Text(Utf8Bytes::from(message_to_send)))
                            .await
                        {
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
            let data_len = data.len() as i64;

            let path_guard = file_save_path_arc.lock().await;
            if path_guard.is_empty() {
                continue;
            }

            if file_handle.is_none() {
                let f = tokio::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&*path_guard)
                    .await;

                if let Ok(file) = f {
                    file_handle = Some(file);
                } else {
                    println!("Failed to create file: {:?}", f.err());
                    break;
                }
            }

            // Write data asynchronously
            let mut successful_write = false;
            if let Some(file) = file_handle.as_mut() {
                if file.write_all(&data).await.is_ok() {
                    successful_write = true;
                }
            }

            drop(path_guard);

            file_processed += data_len;

                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit(
                        "e_p2p",
                        json!({
                            "event": "file_data",
                            "data": {
                                "id": file_id,
                                "chunk_size": data_len,
                                "total_processed": file_processed,
                                "successful_write": successful_write
                            }
                        }),
                    );
                }

                let mut sender_lock = ws_sender_arc.lock().await;
                let _ = sender_lock
                    .send(Message::Text(Utf8Bytes::from(format!(
                        "__processed<|>{}<|>{}<|>{}<|>{}",
                        file_id,
                        data_len,
                        file_processed,
                        successful_write
                    ))))
                    .await;

                if file_processed >= file_size {
                    println!("Finished processing file! Waiting for close.");
                    
                    if let Some(mut file) = file_handle.take() {
                        let _ = file.flush().await;
                    }
                }

            continue;
        }

        // Handle normal text data.
        if state == "text" && msg.is_text() {
            // Send the data to the main window.
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit(
                    "e_p2p",
                    json!({
                        "event": "text_received",
                        "data": {
                            "text": msg_text,
                            "peer": peer
                        }
                    }),
                );
            }

            state = "".to_string();
            continue;
        }
    }

    println!("handle_connection loop for {} exited.", peer);

    Ok(())
}
