use std::sync::atomic::AtomicUsize;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::client::JSONRPC_VERSION;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub version: &'static str,
    pub id: AtomicUsize,
    pub result: Option<Box<RawValue>>,
}

impl JsonRpcResponse {
    pub fn new(&self, id: AtomicUsize, result: Option<Box<RawValue>>) -> Self {
        Self {
            version: JSONRPC_VERSION,
            id,
            result,
        }
    }
}
