use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Environment, observability::tracing::init_traces};

pub fn init_logs_and_tracing(config: &Environment) -> Option<SdkTracerProvider> {
    let filter = LevelFilter::INFO;
    let mut tracing_provider = None;

    let tracer = if let Some(collector_url) = &config.collector_url
        && config.with_tracing
    {
        let sdk_tracer_provider = init_traces(&format!("{}/traces", collector_url));
        let t = sdk_tracer_provider.tracer("Migrator tracing");
        tracing_provider = Some(sdk_tracer_provider.clone());
        Some(t)
    } else {
        None
    };

    if config.json_logs {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .json()
            .flatten_event(true)
            .with_filter(filter);
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(tracer.map(OpenTelemetryLayer::new))
            .init();
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter);
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(tracer.map(OpenTelemetryLayer::new))
            .init();
    }

    tracing_provider
}
