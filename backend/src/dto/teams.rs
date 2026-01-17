use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTeamReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateTeamReq {
    pub name: String,
}

#[derive(Serialize)]
pub struct TeamResp {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct AddTeamMemberReq {
    pub user_id: i64,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct TeamMemberResp {
    pub team_id: i64,
    pub user_id: i64,
    pub role: String,
}
