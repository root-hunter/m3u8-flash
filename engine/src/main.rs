use m3u8_flash_core::output::export::Export;
use m3u8_flash_core::protocol::playlist::Playlist;
use serde_json;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9999".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server in ascolto su ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Errore durante handshake");

            println!("Nuova connessione WebSocket accettata");

            // Creiamo un canale per inviare messaggi tra il thread e Tokio
            let (tx, mut rx) = mpsc::channel::<String>(32);

            let (mut write, mut read) = ws_stream.split();

            // Task per gestire la ricezione e lettura dei messaggi WebSocket
            let writer_task = tokio::spawn(async move {
                while let Some(message) = rx.recv().await {
                    if let Err(e) = write.send(message.into()).await {
                        eprintln!("Errore nell'invio del messaggio: {}", e);
                        break;
                    }
                }
            });

            // Legge i messaggi ricevuti e li rimanda indietro (echo server)
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(msg) if msg.is_text() || msg.is_binary() => {
                        let data = msg.to_text().unwrap();
                        let playlist_url = Arc::new(data.to_string()); // Usa Arc per garantire sicurezza nel thread

                        println!("Messaggio ricevuto: {}", playlist_url);

                        let playlist_url_clone = Arc::clone(&playlist_url);
                        let tx_clone = tx.clone();

                        thread::spawn(move || {
                            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                            let uid = now.as_millis().to_string();

                            let mut library = Playlist::new(playlist_url_clone.to_string());
                            library.scan().unwrap();

                            let payload = serde_json::to_string_pretty(&library).unwrap();
                            let output_base_path = Path::new("./generated");

                            let mut video = library.playlists[1].clone();
                            let folder_path = output_base_path.join("video");
                            
                            let mut audio = None;

                            if library.audios.len() > 0 {
                                audio = Some(library.audios[0].clone());
                            }

                            println!("Audio: {:?}", audio);
                            let target_filename = "video.ts".to_string();
                            video.save(&folder_path, target_filename).unwrap();
                            
                            if audio.is_some() {
                                let mut audio = audio.clone().unwrap();
                                audio.save(output_base_path).unwrap();
                            }

                            let mut export = Export {
                                video: video,
                                audio: audio,
                            };

                            let target_file = output_base_path.join(format!("{:?}.mp4", now));

                            export.save(&target_file).unwrap();

                            tx_clone
                                .blocking_send(payload)
                                .expect("Errore nell'invio al canale");
                        });
                    }
                    Ok(_) => break,
                    Err(e) => {
                        println!("Errore durante la lettura: {}", e);
                        break;
                    }
                }
            }
            println!("Connessione WebSocket chiusa");
        });
    }
}
