use std::sync::atomic::{AtomicUsize, Ordering};

use serde::Serialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use super::{client::JSONRPC_VERSION, method::JsonRpcMethod};

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    pub version: &'static str,
    pub id: AtomicUsize,
    pub method: JsonRpcMethod,
    pub params: Option<Box<Value>>,
}

impl JsonRpcRequest {
    pub fn new(method: JsonRpcMethod, params: Option<Box<Value>>) -> Self {
        Self {
            version: JSONRPC_VERSION,
            id: AtomicUsize::new(1),
            method,
            params,
        }
    }
}

impl Into<Message> for JsonRpcRequest {
    fn into(self) -> Message {
        let id = self.id.fetch_add(1, Ordering::Relaxed);

        Message::Text(
            serde_json::json!({
                "jsonrpc": self.version,
                "id": id,
                "method": self.method.as_str(),
                "params": self.params,
            })
            .to_string()
            .into(),
        )
    }
}
