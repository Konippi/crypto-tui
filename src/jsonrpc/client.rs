use std::sync::Arc;

use anyhow::Ok;
use futures_util::{
    lock::Mutex,
    sink::SinkExt,
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::{net::TcpStream, sync::broadcast::Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::jsonrpc::error::JsonRpcClientError;

use super::request::JsonRpcRequest;

pub const JSONRPC_VERSION: &str = "2.0";

#[derive(Debug)]
pub struct JsonRpcClient {
    sender: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl JsonRpcClient {
    /// Create a new JSON-RPC client over WebSocket
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|err| JsonRpcClientError::Connection(err.to_string()))?;

        tracing::info!("connected to {} successfully!", url);

        let (writer, reader) = ws_stream.split();

        Ok(Self {
            sender: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
        })
    }
}

impl JsonRpcClient {
    pub async fn send_request(&self, request: JsonRpcRequest) -> anyhow::Result<()> {
        let message = request.into();
        let mut sender = self.sender.lock().await;

        sender
            .send(message)
            .await
            .map_err(|err| JsonRpcClientError::Send(err.to_string()))?;

        Ok(())
    }

    pub async fn spawn_message_handler(&self, tx: Sender<Message>) {
        let reader = self.reader.clone();
        tokio::spawn(async move {
            let mut reader = reader.lock().await;

            while let Some(message) = reader.next().await {
                match message {
                    anyhow::Result::Ok(msg) => {
                        if let Err(err) = tx.send(msg) {
                            tracing::error!("Failed to send message through channel: {:?}", err);
                            break;
                        }
                    }
                    anyhow::Result::Err(err) => {
                        tracing::error!("Error reading message: {:?}", err);
                        break;
                    }
                }
            }
        });
    }
}
