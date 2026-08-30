use axum::middleware::from_fn_with_state;
use projects_service::features::common::ProjectState;
use sqlx::MySqlPool;
use tracing_subscriber::EnvFilter;
use test_user_data::insert_test_user_data;

use config::Environment;
use mw_validate_jwt::mw_validate_access_token;
use router::init_router;
use shutdown::shutdown_task;
use {
    axum_server::Handle,
    std::{
        net::{Ipv4Addr, SocketAddr},
        str::FromStr,
        sync::Arc,
    },
    tokio::select,
};

pub mod config;
pub mod mw_validate_jwt;
pub mod router;
pub mod shutdown;
pub mod test_user_data;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::from_filename("../.env") {
        tracing::debug!("Dotenv import failed: {}. Fine for docker", e);
    };
    let env = Environment::load_env().expect("Loading environment variables");
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let mysql_connection_string = format!(
        "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
        env.mysql_user, env.mysql_password, env.db_address, env.mysql_port, env.mysql_database
    );
    tracing::debug!("Connecting to database with url: {0}:{1}", env.db_address, env.mysql_port);
    let pool = MySqlPool::connect(&mysql_connection_string)
        .await.inspect(|_| tracing::debug!("Connected to database"))
        .expect("Failed to connect to database");

    let state = Arc::new(ProjectState {
        pool: pool.clone(),
        token_secret: env.token_secret,
    });
    let mut router = init_router(state.clone());
    if env.test_user_data {
        tracing::warn!("Using test user data");
        router = insert_test_user_data(router);
    } else {
        router = router.layer(from_fn_with_state(state.clone(), mw_validate_access_token));
    }
    let handle = Handle::new();
    let ipv4 = Ipv4Addr::from_str(&env.app_address)
        .expect("Should parse server address in format \"0.0.0.0\"");
    tracing::info!("Starting at address: {}:{}", ipv4, env.app_port);
    let app_task = tokio::spawn(
        axum_server::bind(SocketAddr::from((ipv4, env.app_port)))
            .handle(handle.clone())
            .serve(router.into_make_service()),
    );
    let st = shutdown_task(handle, state);
    select! {
        _ = st => {},
        _ = app_task => {}
    };
}
