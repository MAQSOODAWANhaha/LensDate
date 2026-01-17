use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order_id: i64,
    pub payer_id: i64,
    pub payee_id: i64,
    pub amount: Decimal,
    pub status: String,
    pub pay_channel: String,
    pub stage: Option<String>,
    pub paid_at: Option<DateTimeWithTimeZone>,
    pub proof_url: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
