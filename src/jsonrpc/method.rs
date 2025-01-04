use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum JsonRpcMethod {
    Subscribe,
    Unsubscribe,
}

impl JsonRpcMethod {
    pub fn as_str(&self) -> &str {
        match self {
            JsonRpcMethod::Subscribe => "subscribe",
            JsonRpcMethod::Unsubscribe => "unsubscribe",
        }
    }
}
