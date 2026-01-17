use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "merchant_invoices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub merchant_id: i64,
    pub order_id: Option<i64>,
    pub title: String,
    pub tax_no: Option<String>,
    pub amount: Decimal,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
