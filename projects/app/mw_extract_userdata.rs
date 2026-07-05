use axum::{
    Json, Router,
    extract::Request,
    http::StatusCode,
    middleware::{Next, from_fn},
    response::{IntoResponse, Response},
};
use projects_service::features::common::UserData;
use serde_json::json;

pub fn use_user_data_mw(router: Router) -> Router {
    let router = router.layer(from_fn(mw_extract_user_headers));
    router
}

pub async fn mw_extract_user_headers(mut req: Request, next: Next) -> Response {
    // --- X-ID ---
    let id_header = match req.headers().get("X-ID") {
        Some(v) => v,
        None => {
            return err(
                "MISSING_X_ID",
                "Header X-ID is required",
                StatusCode::UNAUTHORIZED,
            );
        }
    };

    let id_str = match id_header.to_str() {
        Ok(v) => v,
        Err(_) => {
            return err(
                "INVALID_X_ID",
                "Header X-ID must be valid UTF-8",
                StatusCode::BAD_REQUEST,
            );
        }
    };

    let id: u64 = match id_str.parse() {
        Ok(v) => v,
        Err(_) => {
            return err(
                "INVALID_X_ID",
                "Header X-ID must be a valid u64",
                StatusCode::BAD_REQUEST,
            );
        }
    };

    // --- X-ROLE ---
    let role_header = match req.headers().get("X-ROLE") {
        Some(v) => v,
        None => {
            return err(
                "MISSING_X_ROLE",
                "Header X-ROLE is required",
                StatusCode::UNAUTHORIZED,
            );
        }
    };

    let role = match role_header.to_str() {
        Ok(v) => v.to_string(),
        Err(_) => {
            return err(
                "INVALID_X_ROLE",
                "Header X-ROLE must be valid UTF-8",
                StatusCode::BAD_REQUEST,
            );
        }
    };

    // --- Insert into extensions ---
    let user = UserData { id, role };
    req.extensions_mut().insert(user);

    // Continue to next middleware / handler
    next.run(req).await
}
fn err(error: &str, message: &str, code: StatusCode) -> Response {
    (
        code,
        Json(json!({
            "error": error,
            "message": message
        })),
    )
        .into_response()
}
