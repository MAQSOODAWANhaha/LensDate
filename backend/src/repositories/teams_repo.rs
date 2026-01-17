use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};

use crate::entity::{team_members, teams};

pub async fn create_team(
    orm: &DatabaseConnection,
    owner_user_id: i64,
    name: String,
    status: String,
) -> anyhow::Result<teams::Model> {
    let model = teams::ActiveModel {
        owner_user_id: Set(owner_user_id),
        name: Set(name),
        status: Set(status),
        ..Default::default()
    };

    Ok(model.insert(orm).await?)
}

pub async fn list_owned_team_ids(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Vec<i64>> {
    let rows = teams::Entity::find()
        .select_only()
        .column(teams::Column::Id)
        .filter(teams::Column::OwnerUserId.eq(user_id))
        .into_tuple::<i64>()
        .all(orm)
        .await?;

    Ok(rows)
}

pub async fn list_team_members_by_user(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Vec<team_members::Model>> {
    Ok(team_members::Entity::find()
        .filter(team_members::Column::UserId.eq(user_id))
        .all(orm)
        .await?)
}

pub async fn list_teams_by_ids(
    orm: &DatabaseConnection,
    team_ids: Vec<i64>,
) -> anyhow::Result<Vec<teams::Model>> {
    if team_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(teams::Entity::find()
        .filter(teams::Column::Id.is_in(team_ids))
        .order_by_desc(teams::Column::CreatedAt)
        .all(orm)
        .await?)
}

pub async fn find_team_by_id(
    orm: &DatabaseConnection,
    team_id: i64,
) -> anyhow::Result<Option<teams::Model>> {
    Ok(teams::Entity::find_by_id(team_id).one(orm).await?)
}

pub async fn update_team_name(
    orm: &DatabaseConnection,
    team: teams::Model,
    name: String,
) -> anyhow::Result<teams::Model> {
    let mut model: teams::ActiveModel = team.into();
    model.name = Set(name);
    model.updated_at = Set(chrono::Utc::now().into());
    Ok(model.update(orm).await?)
}

pub async fn find_team_member(
    orm: &DatabaseConnection,
    team_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<team_members::Model>> {
    Ok(team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(team_id))
        .filter(team_members::Column::UserId.eq(user_id))
        .one(orm)
        .await?)
}

pub async fn list_team_members(
    orm: &DatabaseConnection,
    team_id: i64,
) -> anyhow::Result<Vec<team_members::Model>> {
    Ok(team_members::Entity::find()
        .filter(team_members::Column::TeamId.eq(team_id))
        .order_by_desc(team_members::Column::UserId)
        .all(orm)
        .await?)
}

pub async fn create_team_member(
    orm: &DatabaseConnection,
    team_id: i64,
    user_id: i64,
    role: String,
) -> anyhow::Result<team_members::Model> {
    let model = team_members::ActiveModel {
        team_id: Set(team_id),
        user_id: Set(user_id),
        role: Set(role),
    };
    Ok(model.insert(orm).await?)
}

pub async fn delete_team_member(
    orm: &DatabaseConnection,
    team_id: i64,
    user_id: i64,
) -> anyhow::Result<()> {
    team_members::Entity::delete_many()
        .filter(team_members::Column::TeamId.eq(team_id))
        .filter(team_members::Column::UserId.eq(user_id))
        .exec(orm)
        .await?;
    Ok(())
}
