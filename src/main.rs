mod config;
mod crypto;
mod jsonrpc;
mod logging;

use config::CONFIG;
use crossbeam_channel::unbounded;
use jsonrpc::{client::JsonRpcClient, method::JsonRpcMethod, request::JsonRpcRequest};
use serde_json::json;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    logging::register_subscriber();

    // Create a channel to communicate between the main thread and the WebSocket task
    let (tx, rx) = unbounded::<Message>();

    let client = JsonRpcClient::new(CONFIG.ws_url).await?;

    client.start_receiving(tx).await;

    tokio::spawn(async move {
        while let Ok(message) = rx.recv() {
            match message {
                Message::Text(text) => {
                    println!("Received text message: {}", text);
                }
                Message::Binary(data) => {
                    println!("Received binary message: {:?}", data);
                }
                _ => {
                    println!("Received other message: {:?}", message);
                }
            }
        }
    });

    loop {
        let request = JsonRpcRequest::new(
            JsonRpcMethod::Subscribe,
            Some(Box::new(json!({"channel": "lightning_board_BTC_JPY"}))),
        );
        client.send_request(request).await?;
        sleep(Duration::from_millis(CONFIG.poll_interval)).await;
    }
}
