use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub photographer_id: Option<i64>,
    pub team_id: Option<i64>,
    pub demand_id: Option<i64>,
    pub quote_id: Option<i64>,
    pub status: String,
    pub pay_type: String,
    pub deposit_amount: Decimal,
    pub total_amount: Decimal,
    pub service_fee: Decimal,
    pub schedule_start: Option<DateTimeWithTimeZone>,
    pub schedule_end: Option<DateTimeWithTimeZone>,
    pub cancelled_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
