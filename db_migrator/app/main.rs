use {
    crate::config::Environment,
    crate::observability::{metrics::init_metrics, otel::OtelGuard, tracing::init_traces},
    opentelemetry::trace::TracerProvider as _,
    tracing::{info_span, level_filters::LevelFilter},
    tracing_opentelemetry::OpenTelemetryLayer,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer},
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

    let level = tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO);
    let subscriber = tracing_subscriber::registry().with(level);

    let mut tracing_provider = None;
    if config.with_tracing && config.collector_url.is_some() {
        let collector_url = config.collector_url.as_ref().unwrap();
        let sdk_tracer_provider = init_traces(&format!("{}/traces", collector_url));
        let tracer = sdk_tracer_provider.tracer("Migrator tracing");
        subscriber.with(OpenTelemetryLayer::new(tracer)).init();
        tracing_provider = Some(sdk_tracer_provider);
    } else {
        subscriber.init();
    }

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
