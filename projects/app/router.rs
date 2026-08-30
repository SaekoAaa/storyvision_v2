use axum::routing::get;
use projects_service::features::add_project_member::handler::add_project_member_handler;
use projects_service::features::delete_project::handler::delete_project_handler;
use projects_service::features::get_project_metadata::handler::get_project_metadata_handler;
use projects_service::features::list_project_members::handler::list_project_members_handler;
use projects_service::features::list_projects::handler::list_projects_handler;
use projects_service::features::remove_project_member::handler::remove_member_from_project_handler;
use projects_service::features::update_project_metadata::handler::update_project_metadata_handler;
use projects_service::features::{
    common::ProjectState, create_project::handler::create_project_handler,
};
use {
    axum::{Router, http::StatusCode},
    projects_service::constants::ROUTER_VERSION_PATH,
    std::sync::Arc,
    tower_http::{cors::CorsLayer, trace::TraceLayer},
};

pub fn init_router(state: Arc<ProjectState>) -> Router {
    Router::new()
        .route("/healthcheck", get(async move || StatusCode::OK))
        .nest(
            ROUTER_VERSION_PATH,
            Router::new().nest(
                "/projects",
                Router::new()
                    .route("/", get(list_projects_handler).post(create_project_handler))
                    .route(
                        "/{id}",
                        get(get_project_metadata_handler)
                            .put(update_project_metadata_handler)
                            .delete(delete_project_handler),
                    )
                    .route(
                        "/{id}/members",
                        get(list_project_members_handler)
                            .post(add_project_member_handler)
                            .delete(remove_member_from_project_handler),
                    )
                    .with_state(state.clone()),
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
