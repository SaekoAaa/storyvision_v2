use {
    crate::config::Environment,
    crate::observability::{metrics::init_metrics, otel::OtelGuard, logs::init_logs_and_tracing},
    tracing::info_span,
};
mod observability;
pub mod app;
pub mod apply_tx;
pub mod database;
pub mod config;

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

    let metrics_provider = if config.with_metrics && config.collector_url.is_some() {
        let collector_url = config.collector_url.as_ref().unwrap();
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
