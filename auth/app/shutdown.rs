use axum_server::Handle;

const SHUTDOWN_GRACE_DEFAULT: u64 = 3;
#[cfg(windows)]
pub async fn shutdown_task(
    handle: Handle,
    app_state: Arc<AuthState>,
    // metrics_provider: Option<SdkMeterProvider>,
) {
    let grace_duration = match std::env::var("SHUTDOWN_GRACE_SECS") {
        Ok(secs_str) => secs_str.parse::<u64>().unwrap_or_else(|err| {
            eprintln!(
                "SHUTDOWN_GRACE_SECS must be a valid positive integer. Using default = {}. Error: {}",
                SHUTDOWN_GRACE_DEFAULT, err
            );
            SHUTDOWN_GRACE_DEFAULT
        }),
        Err(_) => SHUTDOWN_GRACE_DEFAULT,
    };
    let res = tokio::signal::ctrl_c().await;
    app_state.pool.close().await;
    // if let Some(metrics_provider) = metrics_provider {
    //     metrics_provider
    //         .shutdown()
    //         .expect("Should flush all metrics")
    // }
    handle.graceful_shutdown(Some(Duration::from_secs(grace_duration)));
    tracing::debug!("Successfully closed");
    res.expect("Failed to install CTRL_C handler");
}

#[cfg(windows)]
use {auth_service::features::common::AuthState, std::sync::Arc};
#[cfg(unix)]
use {auth_service::features::common::AuthState, std::sync::Arc};
#[cfg(unix)]
pub async fn shutdown_task(
    handle: Handle,
    state: Arc<AuthState>,
    // metrics_provider: Option<SdkMeterProvider>,
) {
    use tokio::{
        select,
        signal::unix::{SignalKind, signal},
    };
    let grace_duration = match std::env::var("SHUTDOWN_GRACE_SECS") {
        Ok(secs_str) => secs_str.parse::<u64>().unwrap_or_else(|err| {
            eprintln!(
                "SHUTDOWN_GRACE_SECS must be a valid positive integer. Using default = {}. Error: {}",
                SHUTDOWN_GRACE_DEFAULT, err
            );
            SHUTDOWN_GRACE_DEFAULT
        }),
        Err(_) => SHUTDOWN_GRACE_DEFAULT,
    };
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");
    select! {
        _ = sigterm.recv() => tracing::info!("Shutting down after SIGTERM"),
        _ = sigint.recv() => tracing::info!("Shutting down after SIGINT")
    }

    state.pool.close().await;
    // if let Some(metrics_provider) = metrics_provider {
    //     metrics_provider
    //         .shutdown()
    //         .expect("Should flush all metrics")
    // }
    handle.graceful_shutdown(Some(std::time::Duration::from_secs(grace_duration)));
}
