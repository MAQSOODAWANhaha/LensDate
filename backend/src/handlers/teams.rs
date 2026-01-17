use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::teams::{AddTeamMemberReq, CreateTeamReq, TeamMemberResp, TeamResp, UpdateTeamReq};
use crate::error::ApiResult;
use crate::services::teams_service;
use crate::state::AppState;

pub async fn create_team(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateTeamReq>,
) -> ApiResult<TeamResp> {
    let data = teams_service::create_team(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_teams(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<TeamResp>> {
    let data = teams_service::list_teams(&state, user_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update_team(
    AuthUser { user_id }: AuthUser,
    Path(team_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<UpdateTeamReq>,
) -> ApiResult<TeamResp> {
    let data = teams_service::update_team(&state, user_id, team_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn add_member(
    AuthUser { user_id }: AuthUser,
    Path(team_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<AddTeamMemberReq>,
) -> ApiResult<TeamMemberResp> {
    let data = teams_service::add_member(&state, user_id, team_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_members(
    AuthUser { user_id }: AuthUser,
    Path(team_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<TeamMemberResp>> {
    let data = teams_service::list_members(&state, user_id, team_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn remove_member(
    AuthUser { user_id }: AuthUser,
    Path((team_id, member_id)): Path<(i64, i64)>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<TeamMemberResp> {
    let data = teams_service::remove_member(&state, user_id, team_id, member_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
