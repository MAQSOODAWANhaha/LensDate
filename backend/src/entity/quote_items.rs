use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quote_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub quote_id: i64,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
