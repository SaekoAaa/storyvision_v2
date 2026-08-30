use {
    crate::config::Environment,
    crate::observability::{logs::init_logs_and_tracing, metrics::init_metrics, otel::OtelGuard},
    tracing::info_span,
};
pub mod app;
pub mod apply_tx;
pub mod config;
pub mod database;
mod observability;

fn main() {
    let _ = dotenvy::dotenv();

    let config = match Environment::load_env() {
        Ok(c) => c,
        Err(e) => {
            tracing_subscriber::fmt::init();
            tracing::error!("Failed to load environment configuration: {}", e);
            std::process::exit(1);
        }
    };

    let tracing_provider = init_logs_and_tracing(&config);

    let metrics_provider = if let Some(collector_url) = config.collector_url
        && config.with_metrics
    {
        init_metrics(&format!("{}/metrics", collector_url)).ok()
    } else {
        None
    };

    let _otel_guard = OtelGuard {
        tracer_provider: tracing_provider,
        meter_provider: metrics_provider,
    };

    let main_span = info_span!("app");
    let _g = main_span.enter();
    app::run()
        .inspect_err(|e| tracing::error!("{}", e))
        .unwrap();
    drop(_g);
}
