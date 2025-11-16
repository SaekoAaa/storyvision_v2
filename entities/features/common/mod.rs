use std::sync::Arc;

use neo4rs::Graph;

pub mod api_error;
pub mod api_response;

#[derive(Clone)]
pub struct UserData {
    pub id: u64,
    pub role: String,
    pub projects_list: Vec<u64>,
}

#[derive(Clone)]
pub struct AppState {
    pub graph: Arc<Graph>,
    pub token_secret: String,
}
