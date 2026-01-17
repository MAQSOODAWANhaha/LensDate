use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_asset_versions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub asset_id: i64,
    pub version: i32,
    pub payload: Json,
    pub created_by: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
