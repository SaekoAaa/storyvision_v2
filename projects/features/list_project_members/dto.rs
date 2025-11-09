use serde::{Deserialize, Serialize};

use crate::model::ProjectMember;

#[derive(Debug, Serialize)]
pub struct ProjectMemberResponse {
    pub member_id: u64,
    pub user_id: u64,
    pub user_email: String,
    pub project_id: u64,
    pub project_name: String,
    pub is_owner: bool,
}
impl From<ProjectMember> for ProjectMemberResponse {
    fn from(member: ProjectMember) -> Self {
        ProjectMemberResponse {
            member_id: member.member_id,
            user_id: member.user_id,
            user_email: member.user_email,
            project_id: member.project_id,
            project_name: member.project_name,
            is_owner: member.is_owner,
        }
    }
}
