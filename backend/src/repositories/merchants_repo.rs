use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entity::{
    demands, merchant_approvals, merchant_asset_versions, merchant_assets, merchant_contracts,
    merchant_invoices, merchant_locations, merchant_template_items, merchant_templates,
    merchant_users, merchants, order_items, orders, users,
};

pub async fn create_merchant(
    orm: &DatabaseConnection,
    name: String,
    logo_url: Option<String>,
    brand_color: Option<String>,
    contact_user_id: Option<i64>,
    status: String,
) -> anyhow::Result<merchants::Model> {
    let model = merchants::ActiveModel {
        name: Set(name),
        logo_url: Set(logo_url),
        brand_color: Set(brand_color),
        contact_user_id: Set(contact_user_id),
        status: Set(status),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_merchant_memberships(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Vec<merchant_users::Model>> {
    Ok(merchant_users::Entity::find()
        .filter(merchant_users::Column::UserId.eq(user_id))
        .all(orm)
        .await?)
}

pub async fn list_merchants_by_contact(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Vec<merchants::Model>> {
    Ok(merchants::Entity::find()
        .filter(merchants::Column::ContactUserId.eq(user_id))
        .all(orm)
        .await?)
}

pub async fn list_merchants_by_ids(
    orm: &DatabaseConnection,
    ids: Vec<i64>,
) -> anyhow::Result<Vec<merchants::Model>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(merchants::Entity::find()
        .filter(merchants::Column::Id.is_in(ids))
        .order_by_desc(merchants::Column::CreatedAt)
        .all(orm)
        .await?)
}

pub async fn list_merchants_by_contact_or_ids(
    orm: &DatabaseConnection,
    contact_user_id: i64,
    member_ids: Vec<i64>,
) -> anyhow::Result<Vec<merchants::Model>> {
    let mut condition = Condition::any().add(merchants::Column::ContactUserId.eq(contact_user_id));
    if !member_ids.is_empty() {
        condition = condition.add(merchants::Column::Id.is_in(member_ids));
    }

    Ok(merchants::Entity::find()
        .filter(condition)
        .order_by_desc(merchants::Column::CreatedAt)
        .all(orm)
        .await?)
}

pub async fn find_merchant_by_id(
    orm: &DatabaseConnection,
    merchant_id: i64,
) -> anyhow::Result<Option<merchants::Model>> {
    Ok(merchants::Entity::find_by_id(merchant_id).one(orm).await?)
}

pub async fn find_merchant_user(
    orm: &DatabaseConnection,
    merchant_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<merchant_users::Model>> {
    Ok(merchant_users::Entity::find()
        .filter(merchant_users::Column::MerchantId.eq(merchant_id))
        .filter(merchant_users::Column::UserId.eq(user_id))
        .one(orm)
        .await?)
}

pub async fn list_merchant_users(
    orm: &DatabaseConnection,
    merchant_id: i64,
) -> anyhow::Result<Vec<merchant_users::Model>> {
    Ok(merchant_users::Entity::find()
        .filter(merchant_users::Column::MerchantId.eq(merchant_id))
        .order_by_desc(merchant_users::Column::UserId)
        .all(orm)
        .await?)
}

pub async fn create_merchant_user(
    orm: &DatabaseConnection,
    merchant_id: i64,
    user_id: i64,
    role: String,
) -> anyhow::Result<merchant_users::Model> {
    let model = merchant_users::ActiveModel {
        merchant_id: Set(merchant_id),
        user_id: Set(user_id),
        role: Set(role),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn delete_merchant_user(
    orm: &DatabaseConnection,
    merchant_id: i64,
    user_id: i64,
) -> anyhow::Result<()> {
    merchant_users::Entity::delete_many()
        .filter(merchant_users::Column::MerchantId.eq(merchant_id))
        .filter(merchant_users::Column::UserId.eq(user_id))
        .exec(orm)
        .await?;
    Ok(())
}

pub async fn create_location(
    orm: &DatabaseConnection,
    merchant_id: i64,
    name: String,
    address: Option<String>,
    city_id: Option<i64>,
) -> anyhow::Result<merchant_locations::Model> {
    let model = merchant_locations::ActiveModel {
        merchant_id: Set(merchant_id),
        name: Set(name),
        address: Set(address),
        city_id: Set(city_id),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_locations(
    orm: &DatabaseConnection,
    merchant_id: i64,
) -> anyhow::Result<Vec<merchant_locations::Model>> {
    Ok(merchant_locations::Entity::find()
        .filter(merchant_locations::Column::MerchantId.eq(merchant_id))
        .order_by_desc(merchant_locations::Column::Id)
        .all(orm)
        .await?)
}

pub async fn find_location_by_id(
    orm: &DatabaseConnection,
    merchant_id: i64,
    location_id: i64,
) -> anyhow::Result<Option<merchant_locations::Model>> {
    Ok(merchant_locations::Entity::find_by_id(location_id)
        .filter(merchant_locations::Column::MerchantId.eq(merchant_id))
        .one(orm)
        .await?)
}

pub async fn update_location(
    orm: &DatabaseConnection,
    location: merchant_locations::Model,
    name: String,
    address: Option<String>,
    city_id: Option<i64>,
) -> anyhow::Result<merchant_locations::Model> {
    let mut model: merchant_locations::ActiveModel = location.into();
    model.name = Set(name);
    model.address = Set(address);
    model.city_id = Set(city_id);
    Ok(model.update(orm).await?)
}

pub async fn delete_location(
    orm: &DatabaseConnection,
    location_id: i64,
) -> anyhow::Result<()> {
    merchant_locations::Entity::delete_by_id(location_id)
        .exec(orm)
        .await?;
    Ok(())
}

pub async fn create_template<C: ConnectionTrait>(
    orm: &C,
    merchant_id: i64,
    name: String,
    description: Option<String>,
    delivery_requirements: Option<serde_json::Value>,
) -> anyhow::Result<merchant_templates::Model> {
    let model = merchant_templates::ActiveModel {
        merchant_id: Set(merchant_id),
        name: Set(name),
        description: Set(description),
        delivery_requirements: Set(delivery_requirements),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn create_template_item<C: ConnectionTrait>(
    orm: &C,
    template_id: i64,
    name: String,
    quantity: i32,
    price: sea_orm::prelude::Decimal,
) -> anyhow::Result<merchant_template_items::Model> {
    let model = merchant_template_items::ActiveModel {
        template_id: Set(template_id),
        name: Set(name),
        quantity: Set(quantity),
        price: Set(price),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_templates_by_merchant_ids(
    orm: &DatabaseConnection,
    merchant_ids: Vec<i64>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<merchant_templates::Model>, u64)> {
    if merchant_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let query = merchant_templates::Entity::find()
        .filter(merchant_templates::Column::MerchantId.is_in(merchant_ids));
    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let offset = (page - 1) * page_size;
    let rows = query
        .order_by_desc(merchant_templates::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn list_template_items_by_template_ids(
    orm: &DatabaseConnection,
    template_ids: Vec<i64>,
) -> anyhow::Result<Vec<merchant_template_items::Model>> {
    if template_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(merchant_template_items::Entity::find()
        .filter(merchant_template_items::Column::TemplateId.is_in(template_ids))
        .order_by_desc(merchant_template_items::Column::Id)
        .all(orm)
        .await?)
}

pub async fn create_approval(
    orm: &DatabaseConnection,
    demand_id: i64,
    merchant_id: i64,
    status: String,
    comment: Option<String>,
) -> anyhow::Result<merchant_approvals::Model> {
    let model = merchant_approvals::ActiveModel {
        demand_id: Set(demand_id),
        merchant_id: Set(merchant_id),
        status: Set(status),
        comment: Set(comment),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_approvals_by_merchants(
    orm: &DatabaseConnection,
    merchant_ids: Vec<i64>,
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<merchant_approvals::Model>, u64)> {
    if merchant_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let mut query = merchant_approvals::Entity::find()
        .filter(merchant_approvals::Column::MerchantId.is_in(merchant_ids));
    if let Some(status) = status {
        query = query.filter(merchant_approvals::Column::Status.eq(status));
    }

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let offset = (page - 1) * page_size;
    let rows = query
        .order_by_desc(merchant_approvals::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn create_contract(
    orm: &DatabaseConnection,
    order_id: i64,
    terms: serde_json::Value,
    version: i32,
) -> anyhow::Result<merchant_contracts::Model> {
    let model = merchant_contracts::ActiveModel {
        order_id: Set(order_id),
        terms: Set(terms),
        version: Set(version),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_contracts_by_order_ids(
    orm: &DatabaseConnection,
    order_ids: Vec<i64>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<merchant_contracts::Model>, u64)> {
    if order_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let query = merchant_contracts::Entity::find()
        .filter(merchant_contracts::Column::OrderId.is_in(order_ids));
    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let offset = (page - 1) * page_size;
    let rows = query
        .order_by_desc(merchant_contracts::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn create_invoice(
    orm: &DatabaseConnection,
    merchant_id: i64,
    order_id: Option<i64>,
    title: String,
    tax_no: Option<String>,
    amount: sea_orm::prelude::Decimal,
    status: String,
) -> anyhow::Result<merchant_invoices::Model> {
    let model = merchant_invoices::ActiveModel {
        merchant_id: Set(merchant_id),
        order_id: Set(order_id),
        title: Set(title),
        tax_no: Set(tax_no),
        amount: Set(amount),
        status: Set(status),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_invoices_by_merchants(
    orm: &DatabaseConnection,
    merchant_ids: Vec<i64>,
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<merchant_invoices::Model>, u64)> {
    if merchant_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let mut query = merchant_invoices::Entity::find()
        .filter(merchant_invoices::Column::MerchantId.is_in(merchant_ids));
    if let Some(status) = status {
        query = query.filter(merchant_invoices::Column::Status.eq(status));
    }

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let offset = (page - 1) * page_size;
    let rows = query
        .order_by_desc(merchant_invoices::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn list_demands_by_merchant_ids(
    orm: &DatabaseConnection,
    merchant_ids: Vec<i64>,
) -> anyhow::Result<Vec<demands::Model>> {
    if merchant_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(demands::Entity::find()
        .filter(demands::Column::MerchantId.is_in(merchant_ids))
        .all(orm)
        .await?)
}

pub async fn list_orders_by_demand_ids(
    orm: &DatabaseConnection,
    demand_ids: Vec<i64>,
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<orders::Model>, u64)> {
    if demand_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let offset = (page - 1) * page_size;
    let mut query = orders::Entity::find().filter(orders::Column::DemandId.is_in(demand_ids));
    if let Some(status) = status {
        query = query.filter(orders::Column::Status.eq(status));
    }

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query
        .order_by_desc(orders::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn list_order_ids_by_demand_ids(
    orm: &DatabaseConnection,
    demand_ids: Vec<i64>,
) -> anyhow::Result<Vec<i64>> {
    if demand_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(orders::Entity::find()
        .select_only()
        .column(orders::Column::Id)
        .filter(orders::Column::DemandId.is_in(demand_ids))
        .into_tuple::<i64>()
        .all(orm)
        .await?)
}

pub async fn find_order_by_id(
    orm: &DatabaseConnection,
    order_id: i64,
) -> anyhow::Result<Option<orders::Model>> {
    Ok(orders::Entity::find_by_id(order_id).one(orm).await?)
}

pub async fn create_merchant_asset<C: ConnectionTrait>(
    orm: &C,
    merchant_id: i64,
    asset_type: String,
    name: String,
    status: String,
) -> anyhow::Result<merchant_assets::Model> {
    let model = merchant_assets::ActiveModel {
        merchant_id: Set(merchant_id),
        asset_type: Set(asset_type),
        name: Set(name),
        status: Set(status),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn list_merchant_assets(
    orm: &DatabaseConnection,
    merchant_id: i64,
    asset_type: Option<String>,
    status: Option<String>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<merchant_assets::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let mut query = merchant_assets::Entity::find()
        .filter(merchant_assets::Column::MerchantId.eq(merchant_id));
    if let Some(asset_type) = asset_type {
        query = query.filter(merchant_assets::Column::AssetType.eq(asset_type));
    }
    if let Some(status) = status {
        query = query.filter(merchant_assets::Column::Status.eq(status));
    }

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query
        .order_by_desc(merchant_assets::Column::UpdatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn find_merchant_asset_by_id(
    orm: &DatabaseConnection,
    asset_id: i64,
) -> anyhow::Result<Option<merchant_assets::Model>> {
    Ok(merchant_assets::Entity::find_by_id(asset_id).one(orm).await?)
}

pub async fn list_asset_versions(
    orm: &DatabaseConnection,
    asset_id: i64,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<merchant_asset_versions::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let query = merchant_asset_versions::Entity::find()
        .filter(merchant_asset_versions::Column::AssetId.eq(asset_id));

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query
        .order_by_desc(merchant_asset_versions::Column::Version)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn list_latest_asset_versions_by_asset_ids(
    orm: &DatabaseConnection,
    asset_ids: Vec<i64>,
) -> anyhow::Result<Vec<merchant_asset_versions::Model>> {
    if asset_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(merchant_asset_versions::Entity::find()
        .filter(merchant_asset_versions::Column::AssetId.is_in(asset_ids))
        .order_by_desc(merchant_asset_versions::Column::Version)
        .all(orm)
        .await?)
}

pub async fn find_latest_asset_version<C: ConnectionTrait>(
    orm: &C,
    asset_id: i64,
) -> anyhow::Result<Option<merchant_asset_versions::Model>> {
    Ok(merchant_asset_versions::Entity::find()
        .filter(merchant_asset_versions::Column::AssetId.eq(asset_id))
        .order_by_desc(merchant_asset_versions::Column::Version)
        .one(orm)
        .await?)
}

pub async fn create_asset_version<C: ConnectionTrait>(
    orm: &C,
    asset_id: i64,
    version: i32,
    payload: serde_json::Value,
    created_by: i64,
) -> anyhow::Result<merchant_asset_versions::Model> {
    let model = merchant_asset_versions::ActiveModel {
        asset_id: Set(asset_id),
        version: Set(version),
        payload: Set(payload),
        created_by: Set(created_by),
        ..Default::default()
    };
    Ok(model.insert(orm).await?)
}

pub async fn touch_merchant_asset_updated_at<C: ConnectionTrait>(
    orm: &C,
    asset_id: i64,
) -> anyhow::Result<merchant_assets::Model> {
    let asset = merchant_assets::Entity::find_by_id(asset_id)
        .one(orm)
        .await?
        .ok_or_else(|| anyhow::anyhow!("merchant_asset_not_found"))?;
    let mut model: merchant_assets::ActiveModel = asset.into();
    model.updated_at = Set(Utc::now().into());
    Ok(model.update(orm).await?)
}

pub async fn list_order_items_by_order_id(
    orm: &DatabaseConnection,
    order_id: i64,
) -> anyhow::Result<Vec<order_items::Model>> {
    Ok(order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(orm)
        .await?)
}

pub async fn get_user_phone_by_id(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Option<String>> {
    Ok(users::Entity::find_by_id(user_id)
        .select_only()
        .column(users::Column::Phone)
        .into_tuple::<String>()
        .one(orm)
        .await?)
}

pub async fn find_demand_by_id(
    orm: &DatabaseConnection,
    demand_id: i64,
) -> anyhow::Result<Option<demands::Model>> {
    Ok(demands::Entity::find_by_id(demand_id).one(orm).await?)
}
