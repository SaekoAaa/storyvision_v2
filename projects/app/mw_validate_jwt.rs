use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::TokenData;
use projects_service::features::common::{ProjectState, UserData};
use serde_json::json;
use projects_service::features::crypto::jwt::{validate_jwt_token, JWTClaims};

pub async fn mw_validate_access_token(
    State(app_state): State<Arc<ProjectState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_header = match request.headers().get("Authorization") {
        Some(v) => v.to_str().unwrap_or(""),
        None => return unauthorized("NO_TOKEN", "Authorization header not found"),
    };
    tracing::debug!(auth_header);
    if !auth_header.starts_with("Bearer ") {
        return unauthorized(
            "INVALID_FORMAT",
            "Authorization header must be Bearer <token>",
        );
    }

    let token = &auth_header["Bearer ".len()..];
    tracing::debug!(token);
    let token_data: TokenData<JWTClaims> = match validate_jwt_token(token, &app_state.token_secret)
    {
        Ok(data) => data,
        Err(e) => {
            tracing::debug!("{:?}", e.into_kind());
            return unauthorized("INVALID_TOKEN", "Token validation failed");
        }
    };

    let claims = token_data.claims;

    let user_data = UserData {
        id: claims.sub,
        role: claims.role,
    };

    request.extensions_mut().insert(user_data);
    tracing::debug!("Finished validation");

    // 5. Передаём дальше
    next.run(request).await
}
fn unauthorized(code: &str, message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "error": code,
            "message": message
        })),
    )
        .into_response()
}
