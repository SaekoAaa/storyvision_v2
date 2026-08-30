use {
    serde::{Deserialize, Serialize},
    std::borrow::Cow,
    utoipa::ToSchema,
};

#[derive(Deserialize, validator::Validate, ToSchema)]
pub struct RegisterUserRequest<'a> {
    #[serde(borrow)]
    #[validate(email, length(min = 5, max = 30))]
    #[schema(
        example = "user@example.com",
        max_length = 30,
        min_length = 5,
        default = "user@example.com",
        pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$"
    )]
    pub email: Cow<'a, str>,
    #[serde(borrow)]
    #[validate(length(min = 8, max = 30))]
    #[schema(
        example = "password12345",
        max_length = 8,
        min_length = 30,
        default = "password12345"
    )]
    pub password: Cow<'a, str>,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterUserResponse {
    pub id: u64,
    pub email: String,
    pub access_token: String,
}
