use {
    crate::infrastructure::{
        load_env::Environment, openapi::init_openapi, router::init_router, shutdown::shutdown_task,
    },
    argon2::password_hash::{SaltString, rand_core::OsRng},
    auth_service::{db::init_database, features::common::AuthState},
    axum_server::Handle,
    std::{
        net::{Ipv4Addr, SocketAddr},
        str::FromStr,
        sync::Arc,
    },
    tokio::select,
    tracing::level_filters::LevelFilter,
    uuid::Uuid,
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
    let db = init_database(&mysql_connection_string)
        .await
        .expect("Connecting to database");
    let db_ptr = Arc::new(db);
    let auth_state = Arc::new(AuthState {
        pool: db_ptr.clone(),
        token_secret: Uuid::new_v4().to_string(),
        saltstring: SaltString::generate(OsRng),
    });
    let router = init_openapi(init_router(auth_state.clone()));
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
