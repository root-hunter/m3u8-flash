use serde_json;
use m3u8_flash::m3u8::playlist::Playlist;
use std::sync::Arc;
use std::thread;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url_lib = "https://vixcloud.co/playlist/279808?b=1&token=bc4c72db837a3144711bee4b01cc881c&expires=1742850942";
//     //Library::new(url);
//     let mut library = Library::new(url_lib.to_string());
//     library.scan().unwrap();

//     //println!("{:#?}", library);

//     // Serializza la struct in formato JSON
//     let json_string = serde_json::to_string_pretty(&library).unwrap();

//     // Salva il JSON in un file
//     let mut file = File::create("library.json")?;
//     file.write_all(json_string.as_bytes())?;

//     library.playlists[1].save().unwrap();
//     Ok(())
// }


use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use std::net::SocketAddr;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9999".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server in ascolto su ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Errore durante handshake");

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
                        let url = Arc::new(data.to_string());  // Usa Arc per garantire sicurezza nel thread
                    
                        println!("Messaggio ricevuto: {}", url);

                        let url_clone = Arc::clone(&url);
                        let tx_clone = tx.clone();

                        thread::spawn(move || {
                            let mut library = Playlist::new(url_clone.to_string());
                            library.scan().unwrap();
                            let payload = serde_json::to_string_pretty(&library).unwrap();
                            //println!("{:?}", library);
                            //library.playlists[1].save().unwrap();

                            tx_clone.blocking_send(payload).expect("Errore nell'invio al canale");
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
