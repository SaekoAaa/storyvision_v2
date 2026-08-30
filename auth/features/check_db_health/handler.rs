use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::features::{check_db_health::usecase::get_db_health, common::AuthState};

pub async fn db_healtcheck_handler(State(state): State<Arc<AuthState>>) -> impl IntoResponse {
    match get_db_health(&state.pool).await {
        Ok(msg) => {
            tracing::info!("Database_health {}", msg);
            StatusCode::OK
        }
        Err(err) => {
            tracing::error!("Database health check failed: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
