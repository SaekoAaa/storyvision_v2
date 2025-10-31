use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub member_id: u64,
}
