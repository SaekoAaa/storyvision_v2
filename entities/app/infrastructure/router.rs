use axum::routing::get;
use entities_service::features::{
    common::AppState, create_character::handler::create_character_handler,
    create_connection::handler::create_connection_handler,
    create_event::handler::create_event_handler, create_relation::handler::create_relation_handler,
    get_connections::handler::get_connections_handler, get_events::handler::get_events_handler,
    get_relations::handler::get_relations_handler,
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
            axum::routing::get(async move || StatusCode::OK),
        )
        .nest(
            ROUTER_VERSION_PATH,
            Router::new()
                .nest(
                    "/characters",
                    Router::new()
                        .route(
                            "/",
                            get(list_characters_handler).post(create_character_handler),
                        )
                        .with_state(state.clone()),
                )
                .nest(
                    "/events",
                    Router::new()
                        .route("/", get(get_events_handler).post(create_event_handler))
                        .with_state(state.clone()),
                )
                .nest(
                    "/relations",
                    Router::new().route(
                        "/",
                        get(get_relations_handler)
                            .post(create_relation_handler)
                            .with_state(state.clone()),
                    ),
                )
                .nest(
                    "/connections",
                    Router::new().route(
                        "/",
                        get(get_connections_handler)
                            .post(create_connection_handler)
                            .with_state(state.clone()),
                    ),
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
