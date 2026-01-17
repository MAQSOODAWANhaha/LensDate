use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::entity::configs;

pub async fn find_config_by_key(
    orm: &DatabaseConnection,
    key: &str,
) -> anyhow::Result<Option<configs::Model>> {
    Ok(configs::Entity::find()
        .filter(configs::Column::Key.eq(key))
        .one(orm)
        .await?)
}

pub async fn update_config(
    orm: &DatabaseConnection,
    model: configs::Model,
    value: serde_json::Value,
) -> anyhow::Result<configs::Model> {
    let mut m: configs::ActiveModel = model.into();
    m.value = Set(value);
    Ok(m.update(orm).await?)
}

pub async fn create_config(
    orm: &DatabaseConnection,
    key: String,
    value: serde_json::Value,
) -> anyhow::Result<configs::Model> {
    let m = configs::ActiveModel {
        key: Set(key),
        value: Set(value),
        ..Default::default()
    };
    Ok(m.insert(orm).await?)
}
