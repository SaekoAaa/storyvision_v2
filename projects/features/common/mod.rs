pub mod api_error;
pub mod api_response;
pub struct ProjectState {
    pub pool: sqlx::MySqlPool,
    pub token_secret: String,
}
#[derive(Clone)]
pub struct UserData {
    pub id: u64,
    pub role: String,
}
// impl UserData {
//     pub fn is_admin(&self) -> Result<(), ApiError> {
//         match self.role != "admin" {
//             true => Err(ApiError::PermissionDenied(format!(
//                 "User: {} is not an admin",
//                 self.id
//             ))),
//             false => Ok(()),
//         }
//     }
// }
