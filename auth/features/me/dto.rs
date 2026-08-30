use {serde::Serialize, utoipa::ToSchema};

#[derive(Serialize, ToSchema)]
pub struct GetUserResponse {
    pub email: String,
    pub role: String,
}
