use {utoipa::ToResponse};

#[derive(ToResponse)]
#[response(description = "Unhandled (yet) server error")]
pub struct InternalErrorResponse;

#[derive(utoipa::ToSchema)]
pub struct BaseErrorResponseSchema {
    message: String,
}