use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "portfolio_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub portfolio_id: i64,
    pub url: String,
    pub tags: Option<Json>,
    pub cover_flag: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
