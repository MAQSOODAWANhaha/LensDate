use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_approvals")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub demand_id: i64,
    pub merchant_id: i64,
    pub status: String,
    pub approver_id: Option<i64>,
    pub comment: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
