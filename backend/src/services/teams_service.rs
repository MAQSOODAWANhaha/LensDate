use std::collections::HashMap;

use crate::dto::teams::{AddTeamMemberReq, CreateTeamReq, TeamMemberResp, TeamResp, UpdateTeamReq};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::teams_repo;
use crate::state::AppState;

pub async fn create_team(
    state: &AppState,
    user_id: i64,
    req: CreateTeamReq,
) -> ServiceResult<TeamResp> {
    validate_team_name(&req.name)?;

    let team = teams_repo::create_team(&state.orm, user_id, req.name, "active".to_string()).await?;

    Ok(TeamResp {
        id: team.id,
        name: team.name,
        status: team.status,
        role: "owner".to_string(),
    })
}

pub async fn list_teams(state: &AppState, user_id: i64) -> ServiceResult<Vec<TeamResp>> {
    let mut team_ids = teams_repo::list_owned_team_ids(&state.orm, user_id).await?;

    let member_rows = teams_repo::list_team_members_by_user(&state.orm, user_id).await?;

    let mut role_map: HashMap<i64, String> = HashMap::new();
    for row in member_rows {
        role_map.insert(row.team_id, row.role);
        if !team_ids.contains(&row.team_id) {
            team_ids.push(row.team_id);
        }
    }

    let rows = teams_repo::list_teams_by_ids(&state.orm, team_ids).await?;

    let items = rows
        .into_iter()
        .map(|row| {
            let role = if row.owner_user_id == user_id {
                "owner".to_string()
            } else {
                role_map
                    .get(&row.id)
                    .cloned()
                    .unwrap_or_else(|| "member".to_string())
            };
            TeamResp {
                id: row.id,
                name: row.name,
                status: row.status,
                role,
            }
        })
        .collect();

    Ok(items)
}

pub async fn update_team(
    state: &AppState,
    user_id: i64,
    team_id: i64,
    req: UpdateTeamReq,
) -> ServiceResult<TeamResp> {
    validate_team_name(&req.name)?;

    let team = teams_repo::find_team_by_id(&state.orm, team_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if team.owner_user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    let updated = teams_repo::update_team_name(&state.orm, team, req.name).await?;

    Ok(TeamResp {
        id: updated.id,
        name: updated.name,
        status: updated.status,
        role: "owner".to_string(),
    })
}

pub async fn add_member(
    state: &AppState,
    user_id: i64,
    team_id: i64,
    req: AddTeamMemberReq,
) -> ServiceResult<TeamMemberResp> {
    ensure_team_admin(state, user_id, team_id).await?;

    let role = req.role.unwrap_or_else(|| "member".to_string());
    if !matches!(role.as_str(), "admin" | "member") {
        return Err(DomainError::InvalidRole.into());
    }

    let existing = teams_repo::find_team_member(&state.orm, team_id, req.user_id).await?;
    if existing.is_some() {
        return Err(DomainError::MemberExists.into());
    }

    teams_repo::create_team_member(&state.orm, team_id, req.user_id, role.clone()).await?;

    Ok(TeamMemberResp {
        team_id,
        user_id: req.user_id,
        role,
    })
}

pub async fn list_members(
    state: &AppState,
    user_id: i64,
    team_id: i64,
) -> ServiceResult<Vec<TeamMemberResp>> {
    ensure_team_member(state, user_id, team_id).await?;

    let rows = teams_repo::list_team_members(&state.orm, team_id).await?;

    let items = rows
        .into_iter()
        .map(|row| TeamMemberResp {
            team_id: row.team_id,
            user_id: row.user_id,
            role: row.role,
        })
        .collect();

    Ok(items)
}

pub async fn remove_member(
    state: &AppState,
    user_id: i64,
    team_id: i64,
    member_id: i64,
) -> ServiceResult<TeamMemberResp> {
    ensure_team_admin(state, user_id, team_id).await?;

    let member = teams_repo::find_team_member(&state.orm, team_id, member_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    teams_repo::delete_team_member(&state.orm, team_id, member_id).await?;

    Ok(TeamMemberResp {
        team_id,
        user_id: member.user_id,
        role: member.role,
    })
}

async fn ensure_team_member(
    state: &AppState,
    user_id: i64,
    team_id: i64,
) -> ServiceResult<()> {
    let team = teams_repo::find_team_by_id(&state.orm, team_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if team.owner_user_id == user_id {
        return Ok(());
    }

    let member = teams_repo::find_team_member(&state.orm, team_id, user_id).await?;
    if member.is_some() {
        Ok(())
    } else {
        Err(DomainError::Forbidden.into())
    }
}

async fn ensure_team_admin(
    state: &AppState,
    user_id: i64,
    team_id: i64,
) -> ServiceResult<()> {
    let team = teams_repo::find_team_by_id(&state.orm, team_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if team.owner_user_id == user_id {
        return Ok(());
    }

    let member = teams_repo::find_team_member(&state.orm, team_id, user_id).await?;
    if member.map(|m| m.role == "admin").unwrap_or(false) {
        Ok(())
    } else {
        Err(DomainError::Forbidden.into())
    }
}

fn validate_team_name(name: &str) -> Result<(), DomainError> {
    if name.len() < 2 || name.len() > 50 {
        Err(DomainError::InvalidName)
    } else {
        Ok(())
    }
}
