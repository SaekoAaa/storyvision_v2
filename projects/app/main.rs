use crate::infrastructure::test_user_data::insert_test_user_data;
use projects_service::features::common::ProjectState;
use sqlx::MySqlPool;

use {
    crate::infrastructure::{load_env::Environment, router::init_router, shutdown::shutdown_task},
    axum_server::Handle,
    std::{
        net::{Ipv4Addr, SocketAddr},
        str::FromStr,
        sync::Arc,
    },
    tokio::select,
    tracing::level_filters::LevelFilter,
};

mod infrastructure;
#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::from_filename("../.env") {
        tracing::debug!("Dotenv import failed: {}. Fine for docker", e);
    };
    let env = Environment::load_env().expect("Loading environment variables");
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mysql_connection_string = format!(
        "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
        env.mysql_user, env.mysql_password, env.db_address, env.mysql_port, env.mysql_database
    );
    tracing::debug!(
        "Connecting to database with url: {}",
        mysql_connection_string
    );
    let pool = MySqlPool::connect(&mysql_connection_string)
        .await
        .expect("Failed to connect to database");
    tracing::debug!("Connected to database");
    let auth_state = Arc::new(ProjectState { pool: pool.clone() });
    let mut router = init_router(auth_state.clone());
    if env.test_user_data {
        router = insert_test_user_data(router);
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
    let st = shutdown_task(handle, auth_state);
    select! {
        _ = st => {},
        _ = app_task => {}
    };
}
