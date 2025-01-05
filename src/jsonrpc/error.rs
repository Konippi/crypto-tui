use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcClientError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Send error: {0}")]
    Send(String),
}
