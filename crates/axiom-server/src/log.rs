use tracing_subscriber::{fmt, EnvFilter};

pub fn init(level: &str, color: bool) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    if color {
        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(false)
            .init();
    } else {
        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_ansi(false)
            .init();
    }
}
