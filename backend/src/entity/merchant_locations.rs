use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_locations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub merchant_id: i64,
    pub name: String,
    pub address: Option<String>,
    pub city_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
