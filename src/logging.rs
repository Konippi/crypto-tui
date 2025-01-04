use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn register_subscriber() {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
}
