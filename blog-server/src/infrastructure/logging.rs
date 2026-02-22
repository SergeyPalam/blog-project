use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use super::config::LogConfig;

pub fn init_logging(log_config: &LogConfig) {
    let filter = EnvFilter::new(&log_config.level);

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339()),
        )
        .init();

    tracing::info!("Logging initialized");
}
