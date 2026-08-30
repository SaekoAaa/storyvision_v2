use std::sync::Arc;

use auth_service::{
    features::common::AuthState,
    features::crypto::jwt::{JWTClaims, validate_jwt_token},
    model::UserData,
};
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::TokenData;
use serde_json::json;

pub async fn mw_validate_access_token(
    State(app_state): State<Arc<AuthState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_header = match request.headers().get("Authorization") {
        Some(v) => v.to_str().unwrap_or(""),
        None => return unauthorized("NO_TOKEN", "Authorization header not found"),
    };
    if !auth_header.starts_with("Bearer ") {
        return unauthorized(
            "INVALID_FORMAT",
            "Authorization header must be Bearer <token>",
        );
    }

    let token = &auth_header["Bearer ".len()..];
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
