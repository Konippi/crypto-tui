mod config;
mod crypto;
mod jsonrpc;
mod logging;

use config::CONFIG;
use jsonrpc::{
    client::JsonRpcClient, method::JsonRpcMethod, request::JsonRpcRequest,
    response::JsonRpcResponse,
};
use serde_json::json;
use tokio::{signal, sync::broadcast};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    logging::register_subscriber();

    // Create a channel to communicate between the main thread and the WebSocket task
    let (tx, mut rx) = broadcast::channel::<Message>(16);

    let client = JsonRpcClient::new(CONFIG.ws_url).await?;
    client.spawn_message_handler(tx).await;

    let request = JsonRpcRequest::new(
        JsonRpcMethod::Subscribe,
        Some(Box::new(json!({"channel": "lightning_board_BTC_JPY"}))),
    );
    client.send_request(request).await?;

    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            match message {
                Message::Text(text) => {
                    let res: JsonRpcResponse = serde_json::from_str(&text.to_string()).unwrap();
                    match res {
                        JsonRpcResponse::Success(success) => {
                            println!("Received: {:?}", success);
                        }
                        JsonRpcResponse::Notification(notification) => {
                            if notification.params.message.bids.is_empty() {
                                continue;
                            }
                            println!("{}", notification.params.message.bids[0].price);
                        }
                    }
                }
                _ => {
                    println!("Received other message: {:?}", message);
                }
            }
        }
    });

    // Wait for a shutdown signal (Ctrl+C)
    signal::ctrl_c().await?;
    println!("Received shutdown signal, exiting...");

    Ok(())
}
