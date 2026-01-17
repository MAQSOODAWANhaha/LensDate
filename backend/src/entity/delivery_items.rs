use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "delivery_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub delivery_id: i64,
    pub file_url: String,
    pub version: Option<String>,
    pub note: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
