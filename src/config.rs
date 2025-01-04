use std::sync::LazyLock;

#[derive(Debug)]
pub struct Config {
    pub ws_url: &'static str,
    pub poll_interval: u64,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    ws_url: "wss://ws.lightstream.bitflyer.com/json-rpc",
    poll_interval: 1000,
});
