use axum::routing::get;
use entities_service::features::{
    common::AppState, create_character::handler::create_character_handler,
    create_connection::handler::create_connection_handler,
    create_event::handler::create_event_handler, create_relation::handler::create_relation_handler,
    get_connections::handler::get_connections_handler, get_events::handler::get_events_handler,
    get_project_graph::handler::get_project_graph_handler,
    get_relations::handler::get_relations_handler,
    import_project::handler::import_project_graph_multipart_handler,
    list_characters::handler::list_characters_handler,
};
use {
    axum::{Router, http::StatusCode, routing::post},
    entities_service::constants::ROUTER_VERSION_PATH,
    std::sync::Arc,
    tower_http::{cors::CorsLayer, trace::TraceLayer},
};

pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/healthcheck",
            get(async move || StatusCode::OK),
        )
        .nest(
            ROUTER_VERSION_PATH,
            Router::new().nest(
                "/entities",
                Router::new()
                    .nest(
                        "/{project_id}/characters",
                        Router::new().route(
                            "/",
                            get(list_characters_handler).post(create_character_handler),
                        ),
                    )
                    .nest(
                        "/{project_id}/events",
                        Router::new()
                            .route("/", get(get_events_handler).post(create_event_handler)),
                    )
                    .nest(
                        "/{project_id}/relations",
                        Router::new().route(
                            "/",
                            get(get_relations_handler).post(create_relation_handler),
                        ),
                    )
                    .nest(
                        "/{project_id}/connections",
                        Router::new().route(
                            "/",
                            get(get_connections_handler).post(create_connection_handler),
                        ),
                    )
                    .nest(
                        "/{project_id}/graph",
                        Router::new().route("/project", get(get_project_graph_handler)),
                    )
                    .route(
                        "/{project_id}/import",
                        post(import_project_graph_multipart_handler),
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
