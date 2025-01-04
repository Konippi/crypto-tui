use std::sync::Arc;

use anyhow::Ok;
use crossbeam_channel::Sender;
use futures_util::{
    lock::Mutex,
    sink::SinkExt,
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::request::JsonRpcRequest;

pub const JSONRPC_VERSION: &str = "2.0";

#[derive(Debug)]
pub struct JsonRpcClient {
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl JsonRpcClient {
    /// Create a new JSON-RPC client over WebSocket
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let (socket, _) = connect_async(url).await?;

        tracing::info!("connected to {} successfully!", url);

        let (writer, reader) = socket.split();

        Ok(Self {
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
        })
    }
}

impl JsonRpcClient {
    pub async fn send_request(&self, request: JsonRpcRequest) -> anyhow::Result<()> {
        let message = request.into();
        let mut writer = self.writer.lock().await;

        writer.send(message).await?;

        Ok(())
    }

    pub async fn start_receiving(&self, tx: Sender<Message>) {
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
