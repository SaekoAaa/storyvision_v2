use axum::extract::Request;
use axum::response::Response;
use axum::{body::Bytes, routing::put};
use projects_service::features::add_project_member;
use projects_service::features::delete_project::handler::{delete_project, delete_project_handler};
use projects_service::features::remove_project_member::handler::remove_member_from_project_handler;
use projects_service::features::update_project_metadata::handler::update_project_metadata_handler;
use projects_service::features::{
    common::ProjectState, create_project::handler::create_project_handler,
};
use std::time::Duration;
use tracing::Span;
use tracing_subscriber::fmt::format::Full;
use {
    axum::{Router, http::StatusCode, routing::post},
    projects_service::constants::ROUTER_VERSION_PATH,
    std::sync::Arc,
    tower_http::{cors::CorsLayer, trace::TraceLayer},
};

pub fn init_router(auth_state: Arc<ProjectState>) -> Router {
    Router::new()
        .route(
            "/healthcheck",
            axum::routing::get(async move || StatusCode::OK),
        )
        .nest(
            ROUTER_VERSION_PATH,
            Router::new().nest(
                "/projects",
                Router::new()
                    .route("/create", post(create_project_handler))
                    .route(
                        "/{id}",
                        put(update_project_metadata_handler).delete(delete_project_handler),
                    )
                    .route(
                        "/{id}/members",
                        post(add_project_member).delete(remove_member_from_project_handler),
                    )
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
