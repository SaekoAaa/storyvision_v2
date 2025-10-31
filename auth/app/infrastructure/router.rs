use auth_service::features::check_db_health::handler::db_healtcheck_handler;
use auth_service::features::login_user::handler::handler_login_user;
use auth_service::features::logout_user::handler::handler_logout_user;
use auth_service::features::refresh_token::handler::handler_refresh_token;
use axum::routing::get;
use {
    auth_service::{
        constants::ROUTER_VERSION_PATH,
        features::{common::AuthState, register_user::handler::handler_register_user},
    },
    axum::{Router, http::StatusCode, routing::post},
    std::sync::Arc,
    tower_http::{cors::CorsLayer, trace::TraceLayer},
};

pub fn init_router(auth_state: Arc<AuthState>) -> Router {
    Router::new()
        .route(
            "/healthcheck",
            axum::routing::get(async move || StatusCode::OK),
        )
        .route(
            "/db_healthcheck",
            get(db_healtcheck_handler).with_state(auth_state.clone()),
        )
        .nest(
            ROUTER_VERSION_PATH,
            Router::new().nest(
                "/auth",
                Router::new()
                    .route("/register", post(handler_register_user))
                    .route("/login", post(handler_login_user))
                    .route("/refresh", post(handler_refresh_token))
                    .route("/logout", post(handler_logout_user))
                    .with_state(auth_state.clone()),
            ),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any),
        )
        .layer(TraceLayer::new_for_http())
}
