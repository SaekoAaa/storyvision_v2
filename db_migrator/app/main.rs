use {
    crate::{
        infrastructure::app,
        observability::{metrics::init_metrics, otel::OtelGuard, tracing::init_traces},
    },
    opentelemetry::trace::TracerProvider as _,
    tracing::{info_span, level_filters::LevelFilter},
    tracing_opentelemetry::OpenTelemetryLayer,
    tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt},
};
mod infrastructure;
mod observability;
fn main() {
    let collector_url = "http://127.0.0.1:4329/v1";
    let traces = init_traces(&format!("{}/traces", collector_url));
    let tracer = traces.tracer("Migrator tracing");
    // let telemetry = tracing_opentelemetry::layer()
    // .with_level(true)
    // .with_tracer(tracer);
    let level = tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry()
        .with(level)
        .with(OpenTelemetryLayer::new(tracer))
        .init();
    let metrics = init_metrics(&format!("{}/metrics", collector_url)).unwrap();
    let _otel_guard = OtelGuard {
        tracer_provider: traces,
        meter_provider: metrics,
    };
    if let Err(e) = dotenvy::dotenv() {
        tracing::debug!("Dotenv import 2 failed: {}. Fine for docker", e);
    };
    let main_span = info_span!("app");
    let _g = main_span.enter();
    app::run()
        .inspect_err(|e| tracing::error!("{}", e))
        .unwrap();
    drop(_g);
}
