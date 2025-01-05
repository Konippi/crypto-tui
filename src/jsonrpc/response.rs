use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse {
    Success(SuccessResponse),
    Notification(NotificationMessage),
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub id: u64,
    pub result: bool,
}

#[derive(Debug, Deserialize)]
pub struct NotificationMessage {
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub method: String,
    pub params: Params,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub channel: String,
    pub message: BoardMessage,
}

#[derive(Debug, Deserialize)]
pub struct BoardMessage {
    pub mid_price: f64,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub price: f64,
    pub size: f64,
}
