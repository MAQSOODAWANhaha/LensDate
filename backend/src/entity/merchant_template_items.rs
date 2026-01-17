use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_template_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub template_id: i64,
    pub name: String,
    pub quantity: i32,
    pub price: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
