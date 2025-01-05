use std::sync::LazyLock;

#[derive(Debug)]
pub struct Config {
    pub ws_url: &'static str,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    ws_url: "wss://ws.lightstream.bitflyer.com/json-rpc",
});
