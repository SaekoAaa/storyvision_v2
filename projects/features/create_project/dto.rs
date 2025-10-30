use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, validator::Validate)]
pub struct CreateProjectRequest<'a> {
    #[serde(borrow)]
    #[validate(length(min = 6, max = 32))]
    pub project_name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    pub id: u64,
    pub name: String,
    pub valid_name: String,
    pub owner_id: u64,
    pub description: String,
}
