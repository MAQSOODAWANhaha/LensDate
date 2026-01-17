use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quotes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub demand_id: i64,
    pub photographer_id: Option<i64>,
    pub team_id: Option<i64>,
    pub total_price: Decimal,
    pub status: String,
    pub version: i32,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
