use entities_service::features::common::AppState;

use crate::infrastructure::{init_neo4::init_neo4j, test_user_data::insert_test_user_data};

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

    tracing::debug!("Connected to database");
    let graph = init_neo4j(&env.neo4j_uri, &env.neo4j_user, &env.neo4j_password)
        .await
        .expect("Failed to connect to Neo4j");

    tracing::debug!("Connected to Neo4j");
    let auth_state = Arc::new(AppState {
        graph: Arc::new(graph),
        token_secret: "secret".to_string(),
    });
    let mut router = init_router(auth_state.clone());
    if env.test_user_data {
        tracing::warn!("Using test user data");
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
    let st = shutdown_task(handle);
    select! {
        _ = st => {},
        _ = app_task => {}
    };
}
