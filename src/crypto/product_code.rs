pub enum ProductCode {
    BtcJpy,
}

impl ProductCode {
    pub fn as_str(&self) -> &str {
        match self {
            ProductCode::BtcJpy => "BTC_JPY",
        }
    }
}
