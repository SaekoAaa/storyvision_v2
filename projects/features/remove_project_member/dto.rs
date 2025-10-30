use serde::Deserialize;

#[derive(Deserialize)]
pub struct RemoveProjectMemberRequest {
    pub member_id: u64,
}
