use axum_test::expect_json::__private::serde_trampoline::Serialize;
use utoipa::ToResponse;
use utoipa::ToSchema;

#[derive(ToResponse)]
#[response(description = "Unhandled (yet) server error")]
pub struct InternalErrorResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct BaseErrorResponseSchema {
    pub error: String,
    pub message: String,
}
