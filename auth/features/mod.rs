use utoipa::OpenApi;
pub mod check_db_health;
pub mod common;
pub mod login_user;
pub mod logout_user;
pub mod refresh_token;
pub mod register_user;
#[derive(OpenApi)]
#[openapi(nest((path = "/auth", api = register_user::handler::RegisterUserOpenApi),
(path = "/auth", api = login_user::handler::LoginUserOpenApi),
(path = "/auth", api = logout_user::handler::LogoutUserOpenApi),
(path = "/auth", api = refresh_token::handler::RefreshTokenOpenApi)))]
pub struct AuthOpenApi;
