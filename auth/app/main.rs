use config::Environment;
use openapi::init_openapi;
use router::init_router;
use shutdown::shutdown_task;
use sqlx::MySqlPool;
use tracing_subscriber::EnvFilter;
use {
    argon2::password_hash::SaltString,
    auth_service::features::common::AuthState,
    axum_server::Handle,
    base64::{Engine, engine::general_purpose},
    std::{
        net::{Ipv4Addr, SocketAddr},
        str::FromStr,
        sync::Arc,
    },
    tokio::select,
};

pub mod config;
pub mod database;
pub mod mw_validate_jwt;
pub mod observability;
pub mod openapi;
pub mod router;
pub mod shutdown;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::from_filename("../.env") {
        tracing::debug!("Dotenv import failed: {}. Fine for docker", e);
    };
    let env = Environment::load_env().expect("Loading environment variables");
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let metrics_provider = if let Some(collector_url) = env.collector_url
        && env.with_metrics
    {
        observability::metrics::init_metrics(&format!("{}/metrics", collector_url)).ok()
    } else {
        None
    };

    let _otel_guard = observability::otel::OtelGuard {
        tracer_provider: None,
        meter_provider: metrics_provider,
    };

    let mysql_connection_string = format!(
        "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
        env.mysql_user, env.mysql_password, env.db_address, env.mysql_port, env.mysql_database
    );
    tracing::debug!(
        "Connecting to database with url: {0}:{1}",
        env.db_address,
        env.mysql_port
    );
    let pool = MySqlPool::connect(&mysql_connection_string)
        .await
        .expect("Failed to connect to database");
    let db_ptr = Arc::new(pool);
    let encoded_salt = general_purpose::STANDARD.encode(env.salt);

    let auth_state = Arc::new(AuthState {
        pool: db_ptr.clone(),
        token_secret: env.token_secret,
        saltstring: SaltString::from_b64(&encoded_salt).expect("Should generate salt"),
        secure_cookies: env.secure_cookies,
    });
    let router = init_openapi(init_router(auth_state.clone()));
    let handle = Handle::new();
    let ipv4 = Ipv4Addr::from_str(&env.app_address)
        .expect("Should parse server address in format \"0.0.0.0\"");
    tracing::info!("Starting at address: {}:{}", ipv4, env.app_port);
    let app_task = tokio::spawn(
        axum_server::bind(SocketAddr::from((ipv4, env.app_port)))
            .handle(handle.clone())
            .serve(router.into_make_service_with_connect_info::<SocketAddr>()),
    );
    let st = shutdown_task(handle, auth_state);
    select! {
        _ = st => {},
        _ = app_task => {}
    };
}
