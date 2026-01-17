use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "demands")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub r#type: String,
    pub city_id: Option<i64>,
    pub location: Option<String>,
    pub schedule_start: Option<DateTimeWithTimeZone>,
    pub schedule_end: Option<DateTimeWithTimeZone>,
    pub budget_min: Option<Decimal>,
    pub budget_max: Option<Decimal>,
    pub people_count: Option<i32>,
    pub style_tags: Option<Json>,
    pub status: String,
    pub is_merchant: bool,
    pub merchant_id: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
