use {
    serde::{Deserialize, Serialize},
    std::borrow::Cow,
    utoipa::ToSchema,
};

#[derive(Serialize, ToSchema)]
pub struct GetUserResponse {
    pub email: String,
    pub role: String,
}
