use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize, validator::Validate)]
pub struct UpdateProjectMetadataRequest<'a> {
    #[serde(borrow)]
    #[validate(length(min = 6, max = 32))]
    pub new_project_name: Cow<'a, str>,
    #[serde(borrow)]
    pub new_description: Option<Cow<'a, str>>,
}
