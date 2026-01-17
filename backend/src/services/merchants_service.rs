use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    TransactionTrait,
};

use crate::dto::merchants::{
    AddMerchantMemberReq, ApprovalListItem, ApprovalListQuery, ApprovalResp, ContractListItem,
    ContractListQuery, ContractResp, CreateApprovalReq, CreateContractReq, CreateInvoiceReq,
    CreateLocationReq, CreateMerchantAssetReq, CreateMerchantAssetVersionReq, CreateMerchantReq,
    CreateTemplateReq, InvoiceListItem, InvoiceListQuery, InvoiceResp, MerchantAssetListQuery,
    MerchantAssetResp, MerchantAssetVersionListQuery, MerchantAssetVersionResp, MerchantListItem,
    MerchantLocationResp, MerchantMemberResp, MerchantOrderDetail, MerchantOrderItem,
    MerchantOrderListItem, MerchantOrderQuery, MerchantOrderReportItem, MerchantOrderReportQuery,
    MerchantOrderReportResp, MerchantResp, TemplateDetailResp, TemplateItemResp, TemplateListQuery,
    TemplateResp, UpdateLocationReq,
};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::entity::{orders, payments, refunds};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::merchants_repo;
use crate::state::AppState;

pub async fn create_merchant(
    state: &AppState,
    req: CreateMerchantReq,
) -> ServiceResult<MerchantResp> {
    validate_merchant_name(&req.name)?;

    let inserted = merchants_repo::create_merchant(
        &state.orm,
        req.name,
        req.logo_url,
        req.brand_color,
        Some(req.contact_user_id),
        "pending".to_string(),
    )
    .await?;

    Ok(MerchantResp {
        id: inserted.id,
        name: inserted.name,
        status: inserted.status,
    })
}

pub async fn list_my_merchants(
    state: &AppState,
    user_id: i64,
) -> ServiceResult<Vec<MerchantListItem>> {
    let memberships = merchants_repo::list_merchant_memberships(&state.orm, user_id).await?;

    let mut member_map: HashMap<i64, String> = HashMap::new();
    let mut member_ids: Vec<i64> = Vec::new();
    for row in memberships {
        member_map.insert(row.merchant_id, row.role);
        member_ids.push(row.merchant_id);
    }

    let rows = merchants_repo::list_merchants_by_contact_or_ids(&state.orm, user_id, member_ids)
        .await?;

    let items = rows
        .into_iter()
        .map(|m| {
            let role = if m.contact_user_id == Some(user_id) {
                "contact".to_string()
            } else {
                member_map
                    .get(&m.id)
                    .cloned()
                    .unwrap_or_else(|| "member".to_string())
            };
            MerchantListItem {
                id: m.id,
                name: m.name,
                status: m.status,
                role,
                logo_url: m.logo_url,
                brand_color: m.brand_color,
            }
        })
        .collect();

    Ok(items)
}

pub async fn create_location(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    req: CreateLocationReq,
) -> ServiceResult<MerchantLocationResp> {
    ensure_merchant_manager(state, user_id, merchant_id).await?;
    validate_location(&req.name, req.address.as_ref())?;

    let inserted = merchants_repo::create_location(
        &state.orm,
        merchant_id,
        req.name,
        req.address,
        req.city_id,
    )
    .await?;

    Ok(MerchantLocationResp {
        id: inserted.id,
        merchant_id: inserted.merchant_id,
        name: inserted.name,
        address: inserted.address,
        city_id: inserted.city_id,
    })
}

pub async fn list_locations(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
) -> ServiceResult<Vec<MerchantLocationResp>> {
    ensure_merchant_access(state, user_id, merchant_id).await?;

    let rows = merchants_repo::list_locations(&state.orm, merchant_id).await?;

    Ok(rows
        .into_iter()
        .map(|row| MerchantLocationResp {
            id: row.id,
            merchant_id: row.merchant_id,
            name: row.name,
            address: row.address,
            city_id: row.city_id,
        })
        .collect())
}

pub async fn update_location(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    location_id: i64,
    req: UpdateLocationReq,
) -> ServiceResult<MerchantLocationResp> {
    ensure_merchant_manager(state, user_id, merchant_id).await?;
    validate_location(&req.name, req.address.as_ref())?;

    let location = merchants_repo::find_location_by_id(&state.orm, merchant_id, location_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let updated = merchants_repo::update_location(
        &state.orm,
        location,
        req.name,
        req.address,
        req.city_id,
    )
    .await?;

    Ok(MerchantLocationResp {
        id: updated.id,
        merchant_id: updated.merchant_id,
        name: updated.name,
        address: updated.address,
        city_id: updated.city_id,
    })
}

pub async fn delete_location(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    location_id: i64,
) -> ServiceResult<MerchantLocationResp> {
    ensure_merchant_manager(state, user_id, merchant_id).await?;

    let location = merchants_repo::find_location_by_id(&state.orm, merchant_id, location_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    merchants_repo::delete_location(&state.orm, location_id).await?;

    Ok(MerchantLocationResp {
        id: location.id,
        merchant_id: location.merchant_id,
        name: location.name,
        address: location.address,
        city_id: location.city_id,
    })
}

pub async fn add_member(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    req: AddMerchantMemberReq,
) -> ServiceResult<MerchantMemberResp> {
    let merchant = ensure_merchant_manager(state, user_id, merchant_id).await?;

    let role = req.role.unwrap_or_else(|| "requester".to_string());
    if !matches!(role.as_str(), "requester" | "approver" | "finance") {
        return Err(DomainError::InvalidRole.into());
    }

    if merchant.contact_user_id == Some(req.user_id) {
        return Err(DomainError::ContactUser.into());
    }

    let existing = merchants_repo::find_merchant_user(&state.orm, merchant_id, req.user_id).await?;
    if existing.is_some() {
        return Err(DomainError::MemberExists.into());
    }

    merchants_repo::create_merchant_user(&state.orm, merchant_id, req.user_id, role.clone())
        .await?;

    Ok(MerchantMemberResp {
        merchant_id,
        user_id: req.user_id,
        role,
    })
}

pub async fn list_members(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
) -> ServiceResult<Vec<MerchantMemberResp>> {
    ensure_merchant_access(state, user_id, merchant_id).await?;

    let rows = merchants_repo::list_merchant_users(&state.orm, merchant_id).await?;

    Ok(rows
        .into_iter()
        .map(|row| MerchantMemberResp {
            merchant_id: row.merchant_id,
            user_id: row.user_id,
            role: row.role,
        })
        .collect())
}

pub async fn remove_member(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    member_id: i64,
) -> ServiceResult<MerchantMemberResp> {
    ensure_merchant_manager(state, user_id, merchant_id).await?;

    let member = merchants_repo::find_merchant_user(&state.orm, merchant_id, member_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    merchants_repo::delete_merchant_user(&state.orm, merchant_id, member_id).await?;

    Ok(MerchantMemberResp {
        merchant_id: member.merchant_id,
        user_id: member.user_id,
        role: member.role,
    })
}

pub async fn create_template(
    state: &AppState,
    req: CreateTemplateReq,
) -> ServiceResult<TemplateResp> {
    if req.items.is_empty() {
        return Err(DomainError::ItemsRequired.into());
    }

    let txn = state.orm.begin().await?;

    let inserted = merchants_repo::create_template(
        &txn,
        req.merchant_id,
        req.name,
        req.description,
        req.delivery_requirements,
    )
    .await?;

    for item in req.items {
        merchants_repo::create_template_item(
            &txn,
            inserted.id,
            item.name,
            item.quantity,
            decimal_from_f64(item.price),
        )
        .await?;
    }

    txn.commit().await?;

    Ok(TemplateResp {
        id: inserted.id,
        merchant_id: inserted.merchant_id,
        name: inserted.name,
    })
}

pub async fn list_templates(
    state: &AppState,
    user_id: i64,
    query: TemplateListQuery,
) -> ServiceResult<Paged<TemplateDetailResp>> {
    let merchant_ids = load_accessible_merchant_ids(state, user_id, query.merchant_id).await?;
    if merchant_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) = merchants_repo::list_templates_by_merchant_ids(
        &state.orm,
        merchant_ids.clone(),
        page,
        page_size,
    )
    .await?;
    if rows.is_empty() {
        return Ok(Paged::new(Vec::new(), total, page, page_size));
    }

    let template_ids: Vec<i64> = rows.iter().map(|t| t.id).collect();
    let item_rows =
        merchants_repo::list_template_items_by_template_ids(&state.orm, template_ids).await?;

    let mut item_map: HashMap<i64, Vec<TemplateItemResp>> = HashMap::new();
    for item in item_rows {
        item_map
            .entry(item.template_id)
            .or_default()
            .push(TemplateItemResp {
                name: item.name,
                quantity: item.quantity,
                price: decimal_to_f64(item.price),
            });
    }

    let items = rows
        .into_iter()
        .map(|row| TemplateDetailResp {
            id: row.id,
            merchant_id: row.merchant_id,
            name: row.name,
            description: row.description,
            delivery_requirements: row.delivery_requirements,
            created_at: row.created_at.to_rfc3339(),
            items: item_map.remove(&row.id).unwrap_or_default(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn create_approval(
    state: &AppState,
    req: CreateApprovalReq,
) -> ServiceResult<ApprovalResp> {
    if !matches!(req.status.as_str(), "draft" | "pending" | "approved" | "rejected") {
        return Err(DomainError::InvalidStatus.into());
    }

    let inserted = merchants_repo::create_approval(
        &state.orm,
        req.demand_id,
        req.merchant_id,
        req.status,
        req.comment,
    )
    .await?;

    Ok(ApprovalResp {
        id: inserted.id,
        status: inserted.status,
    })
}

pub async fn list_approvals(
    state: &AppState,
    user_id: i64,
    query: ApprovalListQuery,
) -> ServiceResult<Paged<ApprovalListItem>> {
    let merchant_ids = load_accessible_merchant_ids(state, user_id, query.merchant_id).await?;
    if merchant_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) = merchants_repo::list_approvals_by_merchants(
        &state.orm,
        merchant_ids,
        query.status,
        page,
        page_size,
    )
    .await?;

    let items = rows
        .into_iter()
        .map(|row| ApprovalListItem {
            id: row.id,
            demand_id: row.demand_id,
            merchant_id: row.merchant_id,
            status: row.status,
            approver_id: row.approver_id,
            comment: row.comment,
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn create_contract(
    state: &AppState,
    req: CreateContractReq,
) -> ServiceResult<ContractResp> {
    let version = req.version.unwrap_or(1);
    let inserted =
        merchants_repo::create_contract(&state.orm, req.order_id, req.terms, version).await?;

    Ok(ContractResp {
        id: inserted.id,
        order_id: inserted.order_id,
        version: inserted.version,
    })
}

pub async fn list_contracts(
    state: &AppState,
    user_id: i64,
    query: ContractListQuery,
) -> ServiceResult<Paged<ContractListItem>> {
    let merchant_ids = load_accessible_merchant_ids(state, user_id, query.merchant_id).await?;
    if merchant_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let order_ids = load_accessible_order_ids(state, &merchant_ids).await?;
    if order_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) =
        merchants_repo::list_contracts_by_order_ids(&state.orm, order_ids, page, page_size).await?;

    let items = rows
        .into_iter()
        .map(|row| ContractListItem {
            id: row.id,
            order_id: row.order_id,
            version: row.version,
            created_at: row.created_at.to_rfc3339(),
            terms: row.terms,
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn create_invoice(
    state: &AppState,
    req: CreateInvoiceReq,
) -> ServiceResult<InvoiceResp> {
    if req.amount <= 0.0 {
        return Err(DomainError::InvalidAmount.into());
    }

    let inserted = merchants_repo::create_invoice(
        &state.orm,
        req.merchant_id,
        req.order_id,
        req.title,
        req.tax_no,
        decimal_from_f64(req.amount),
        "pending".to_string(),
    )
    .await?;

    Ok(InvoiceResp {
        id: inserted.id,
        merchant_id: inserted.merchant_id,
        status: inserted.status,
    })
}

pub async fn list_invoices(
    state: &AppState,
    user_id: i64,
    query: InvoiceListQuery,
) -> ServiceResult<Paged<InvoiceListItem>> {
    let merchant_ids = load_accessible_merchant_ids(state, user_id, query.merchant_id).await?;
    if merchant_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) = merchants_repo::list_invoices_by_merchants(
        &state.orm,
        merchant_ids,
        query.status,
        page,
        page_size,
    )
    .await?;

    let items = rows
        .into_iter()
        .map(|row| InvoiceListItem {
            id: row.id,
            merchant_id: row.merchant_id,
            order_id: row.order_id,
            title: row.title,
            tax_no: row.tax_no,
            amount: decimal_to_f64(row.amount),
            status: row.status,
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn create_merchant_asset(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    req: CreateMerchantAssetReq,
) -> ServiceResult<MerchantAssetResp> {
    ensure_merchant_manager(state, user_id, merchant_id).await?;
    validate_asset_name(&req.name)?;

    let asset_type = normalize_asset_type(req.asset_type)?;
    let status = normalize_asset_status(req.status.unwrap_or_else(|| "active".to_string()))?;

    let txn = state.orm.begin().await?;
    let mut asset = merchants_repo::create_merchant_asset(
        &txn,
        merchant_id,
        asset_type,
        req.name,
        status,
    )
    .await?;

    let mut latest_version = None;
    let mut latest_payload = None;
    if let Some(payload) = req.payload {
        let inserted =
            merchants_repo::create_asset_version(&txn, asset.id, 1, payload, user_id).await?;
        latest_version = Some(inserted.version);
        latest_payload = Some(inserted.payload.clone());
        asset = merchants_repo::touch_merchant_asset_updated_at(&txn, asset.id).await?;
    }

    txn.commit().await?;

    Ok(MerchantAssetResp {
        id: asset.id,
        merchant_id: asset.merchant_id,
        asset_type: asset.asset_type,
        name: asset.name,
        status: asset.status,
        latest_version,
        latest_payload,
        updated_at: asset.updated_at.to_rfc3339(),
    })
}

pub async fn list_merchant_assets(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
    query: MerchantAssetListQuery,
) -> ServiceResult<Paged<MerchantAssetResp>> {
    ensure_merchant_access(state, user_id, merchant_id).await?;

    let asset_type = match query.asset_type {
        Some(value) => Some(normalize_asset_type(value)?),
        None => None,
    };
    let status = match query.status {
        Some(value) => Some(normalize_asset_status(value)?),
        None => None,
    };

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) = merchants_repo::list_merchant_assets(
        &state.orm,
        merchant_id,
        asset_type,
        status,
        page,
        page_size,
    )
    .await?;

    if rows.is_empty() {
        return Ok(Paged::new(Vec::new(), total, page, page_size));
    }

    let asset_ids: Vec<i64> = rows.iter().map(|row| row.id).collect();
    let latest_rows =
        merchants_repo::list_latest_asset_versions_by_asset_ids(&state.orm, asset_ids).await?;
    let mut latest_map: HashMap<i64, crate::entity::merchant_asset_versions::Model> =
        HashMap::new();
    for row in latest_rows {
        latest_map.entry(row.asset_id).or_insert(row);
    }

    let items = rows
        .into_iter()
        .map(|row| {
            let latest = latest_map.get(&row.id);
            MerchantAssetResp {
                id: row.id,
                merchant_id: row.merchant_id,
                asset_type: row.asset_type,
                name: row.name,
                status: row.status,
                latest_version: latest.map(|v| v.version),
                latest_payload: latest.map(|v| v.payload.clone()),
                updated_at: row.updated_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn list_merchant_asset_versions(
    state: &AppState,
    user_id: i64,
    asset_id: i64,
    query: MerchantAssetVersionListQuery,
) -> ServiceResult<Paged<MerchantAssetVersionResp>> {
    let asset = ensure_merchant_asset_access(state, user_id, asset_id).await?;
    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) =
        merchants_repo::list_asset_versions(&state.orm, asset.id, page, page_size).await?;

    let items = rows
        .into_iter()
        .map(|row| MerchantAssetVersionResp {
            id: row.id,
            asset_id: row.asset_id,
            version: row.version,
            payload: row.payload,
            created_by: row.created_by,
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn create_merchant_asset_version(
    state: &AppState,
    user_id: i64,
    asset_id: i64,
    req: CreateMerchantAssetVersionReq,
) -> ServiceResult<MerchantAssetVersionResp> {
    let asset = ensure_merchant_asset_manager(state, user_id, asset_id).await?;
    let txn = state.orm.begin().await?;
    let latest = merchants_repo::find_latest_asset_version(&txn, asset.id).await?;
    let next_version = latest.map(|row| row.version + 1).unwrap_or(1);

    let inserted = merchants_repo::create_asset_version(
        &txn,
        asset.id,
        next_version,
        req.payload,
        user_id,
    )
    .await?;
    merchants_repo::touch_merchant_asset_updated_at(&txn, asset.id).await?;
    txn.commit().await?;

    Ok(MerchantAssetVersionResp {
        id: inserted.id,
        asset_id: inserted.asset_id,
        version: inserted.version,
        payload: inserted.payload,
        created_by: inserted.created_by,
        created_at: inserted.created_at.to_rfc3339(),
    })
}

pub async fn list_merchant_orders(
    state: &AppState,
    user_id: i64,
    query: MerchantOrderQuery,
) -> ServiceResult<Paged<MerchantOrderListItem>> {
    let merchant_ids = load_accessible_merchant_ids(state, user_id, query.merchant_id).await?;
    if merchant_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let demand_ids = load_accessible_demand_ids(state, &merchant_ids).await?;
    if demand_ids.is_empty() {
        let (page, page_size) = normalize_pagination(query.page, query.page_size);
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) = merchants_repo::list_orders_by_demand_ids(
        &state.orm,
        demand_ids,
        query.status,
        page,
        page_size,
    )
    .await?;

    let items = rows
        .into_iter()
        .map(|row| MerchantOrderListItem {
            id: row.id,
            status: row.status,
            total_amount: decimal_to_f64(row.total_amount),
            demand_id: row.demand_id,
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn get_merchant_order(
    state: &AppState,
    user_id: i64,
    order_id: i64,
) -> ServiceResult<MerchantOrderDetail> {
    let order = merchants_repo::find_order_by_id(&state.orm, order_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let demand_id = order.demand_id.ok_or(DomainError::Forbidden)?;
    let demand = merchants_repo::find_demand_by_id(&state.orm, demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    let merchant_id = demand.merchant_id.ok_or(DomainError::Forbidden)?;
    ensure_merchant_access(state, user_id, merchant_id).await?;

    let items = merchants_repo::list_order_items_by_order_id(&state.orm, order_id)
        .await?
        .into_iter()
        .map(|it| MerchantOrderItem {
            name: it.name,
            price: decimal_to_f64(it.price),
            quantity: it.quantity,
        })
        .collect();

    let user_phone = merchants_repo::get_user_phone_by_id(&state.orm, order.user_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    Ok(MerchantOrderDetail {
        id: order.id,
        status: order.status,
        pay_type: order.pay_type,
        total_amount: decimal_to_f64(order.total_amount),
        service_fee: decimal_to_f64(order.service_fee),
        schedule_start: order.schedule_start.map(|t| t.to_rfc3339()),
        schedule_end: order.schedule_end.map(|t| t.to_rfc3339()),
        user_id: order.user_id,
        user_phone,
        demand_id: order.demand_id,
        created_at: order.created_at.to_rfc3339(),
        items,
    })
}

pub async fn export_merchant_orders_report(
    state: &AppState,
    user_id: i64,
    query: MerchantOrderReportQuery,
) -> ServiceResult<MerchantOrderReportResp> {
    let merchant_ids = load_accessible_merchant_ids(state, user_id, query.merchant_id).await?;
    let format = query.format.unwrap_or_else(|| "json".to_string());
    if merchant_ids.is_empty() {
        return Ok(MerchantOrderReportResp {
            format,
            generated_at: Utc::now().to_rfc3339(),
            total: 0,
            items: Vec::new(),
            csv: None,
        });
    }

    let demand_ids = load_accessible_demand_ids(state, &merchant_ids).await?;
    if demand_ids.is_empty() {
        return Ok(MerchantOrderReportResp {
            format,
            generated_at: Utc::now().to_rfc3339(),
            total: 0,
            items: Vec::new(),
            csv: None,
        });
    }

    let limit = query.limit.unwrap_or(500).min(5000);
    let mut q = orders::Entity::find().filter(orders::Column::DemandId.is_in(demand_ids));
    if let Some(status) = &query.status {
        q = q.filter(orders::Column::Status.eq(status));
    }
    if let Some(start) = query.start_date.as_deref() {
        let dt = parse_date(start)?;
        q = q.filter(orders::Column::CreatedAt.gte(dt));
    }
    if let Some(end) = query.end_date.as_deref() {
        let dt = parse_date(end)?;
        q = q.filter(orders::Column::CreatedAt.lt(dt + Duration::days(1)));
    }

    let total = q.clone().count(&state.orm).await?;
    let rows = q
        .order_by_desc(orders::Column::CreatedAt)
        .limit(limit)
        .all(&state.orm)
        .await?;

    if rows.is_empty() {
        return Ok(MerchantOrderReportResp {
            format,
            generated_at: Utc::now().to_rfc3339(),
            total,
            items: Vec::new(),
            csv: None,
        });
    }

    let order_ids: Vec<i64> = rows.iter().map(|o| o.id).collect();

    let payment_rows = payments::Entity::find()
        .filter(payments::Column::OrderId.is_in(order_ids.clone()))
        .filter(payments::Column::Status.eq("success"))
        .all(&state.orm)
        .await?;
    let mut paid_map: HashMap<i64, f64> = HashMap::new();
    for row in payment_rows {
        let entry = paid_map.entry(row.order_id).or_insert(0.0);
        *entry += decimal_to_f64(row.amount);
    }

    let refund_rows = refunds::Entity::find()
        .filter(refunds::Column::OrderId.is_in(order_ids))
        .filter(refunds::Column::Status.eq("paid"))
        .all(&state.orm)
        .await?;
    let mut refund_map: HashMap<i64, f64> = HashMap::new();
    for row in refund_rows {
        let entry = refund_map.entry(row.order_id).or_insert(0.0);
        *entry += decimal_to_f64(row.amount);
    }

    let items: Vec<MerchantOrderReportItem> = rows
        .into_iter()
        .map(|row| MerchantOrderReportItem {
            id: row.id,
            demand_id: row.demand_id,
            status: row.status,
            total_amount: decimal_to_f64(row.total_amount),
            paid_amount: paid_map.get(&row.id).cloned().unwrap_or(0.0),
            refund_amount: refund_map.get(&row.id).cloned().unwrap_or(0.0),
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    let csv = if format == "csv" {
        Some(render_orders_csv(&items))
    } else {
        None
    };

    Ok(MerchantOrderReportResp {
        format,
        generated_at: Utc::now().to_rfc3339(),
        total,
        items,
        csv,
    })
}

async fn ensure_merchant_asset_access(
    state: &AppState,
    user_id: i64,
    asset_id: i64,
) -> ServiceResult<crate::entity::merchant_assets::Model> {
    let asset = merchants_repo::find_merchant_asset_by_id(&state.orm, asset_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    ensure_merchant_access(state, user_id, asset.merchant_id).await?;
    Ok(asset)
}

async fn ensure_merchant_asset_manager(
    state: &AppState,
    user_id: i64,
    asset_id: i64,
) -> ServiceResult<crate::entity::merchant_assets::Model> {
    let asset = merchants_repo::find_merchant_asset_by_id(&state.orm, asset_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    ensure_merchant_manager(state, user_id, asset.merchant_id).await?;
    Ok(asset)
}

async fn ensure_merchant_access(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
) -> ServiceResult<crate::entity::merchants::Model> {
    let merchant = merchants_repo::find_merchant_by_id(&state.orm, merchant_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if merchant.contact_user_id == Some(user_id) {
        return Ok(merchant);
    }

    let member = merchants_repo::find_merchant_user(&state.orm, merchant_id, user_id).await?;
    if member.is_some() {
        Ok(merchant)
    } else {
        Err(DomainError::Forbidden.into())
    }
}

async fn ensure_merchant_manager(
    state: &AppState,
    user_id: i64,
    merchant_id: i64,
) -> ServiceResult<crate::entity::merchants::Model> {
    let merchant = merchants_repo::find_merchant_by_id(&state.orm, merchant_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if merchant.contact_user_id == Some(user_id) {
        return Ok(merchant);
    }

    let member = merchants_repo::find_merchant_user(&state.orm, merchant_id, user_id).await?;
    if member.map(|m| m.role == "approver").unwrap_or(false) {
        Ok(merchant)
    } else {
        Err(DomainError::Forbidden.into())
    }
}

async fn load_accessible_merchant_ids(
    state: &AppState,
    user_id: i64,
    merchant_id: Option<i64>,
) -> ServiceResult<Vec<i64>> {
    if let Some(merchant_id) = merchant_id {
        ensure_merchant_access(state, user_id, merchant_id).await?;
        return Ok(vec![merchant_id]);
    }

    let mut ids: HashSet<i64> = HashSet::new();
    let owned = merchants_repo::list_merchants_by_contact(&state.orm, user_id).await?;
    for m in owned {
        ids.insert(m.id);
    }

    let members = merchants_repo::list_merchant_memberships(&state.orm, user_id).await?;
    for m in members {
        ids.insert(m.merchant_id);
    }

    Ok(ids.into_iter().collect())
}

async fn load_accessible_demand_ids(
    state: &AppState,
    merchant_ids: &[i64],
) -> ServiceResult<Vec<i64>> {
    if merchant_ids.is_empty() {
        return Ok(Vec::new());
    }

    let rows = merchants_repo::list_demands_by_merchant_ids(&state.orm, merchant_ids.to_vec())
        .await?;
    Ok(rows.into_iter().map(|d| d.id).collect())
}

async fn load_accessible_order_ids(
    state: &AppState,
    merchant_ids: &[i64],
) -> ServiceResult<Vec<i64>> {
    let demand_ids = load_accessible_demand_ids(state, merchant_ids).await?;
    if demand_ids.is_empty() {
        return Ok(Vec::new());
    }

    let rows = merchants_repo::list_order_ids_by_demand_ids(&state.orm, demand_ids).await?;
    Ok(rows)
}

fn validate_merchant_name(name: &str) -> Result<(), DomainError> {
    if name.len() < 2 || name.len() > 50 {
        Err(DomainError::InvalidName)
    } else {
        Ok(())
    }
}

fn validate_location(name: &str, address: Option<&String>) -> Result<(), DomainError> {
    if name.len() < 2 || name.len() > 50 {
        return Err(DomainError::InvalidName);
    }

    if address.map(|value| value.len() > 200).unwrap_or(false) {
        return Err(DomainError::InvalidAddress);
    }

    Ok(())
}

fn validate_asset_name(name: &str) -> Result<(), DomainError> {
    if name.len() < 2 || name.len() > 100 {
        Err(DomainError::InvalidName)
    } else {
        Ok(())
    }
}

fn normalize_asset_type(value: String) -> Result<String, DomainError> {
    let value = value.to_lowercase();
    if matches!(value.as_str(), "logo" | "brand" | "style" | "reference") {
        Ok(value)
    } else {
        Err(DomainError::BadRequest("invalid_asset_type".to_string()))
    }
}

fn normalize_asset_status(value: String) -> Result<String, DomainError> {
    let value = value.to_lowercase();
    if matches!(value.as_str(), "active" | "archived") {
        Ok(value)
    } else {
        Err(DomainError::BadRequest("invalid_asset_status".to_string()))
    }
}

fn decimal_from_f64(v: f64) -> sea_orm::prelude::Decimal {
    sea_orm::prelude::Decimal::from_str(&v.to_string())
        .unwrap_or(sea_orm::prelude::Decimal::ZERO)
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}

fn parse_date(value: &str) -> ServiceResult<DateTime<Utc>> {
    let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| DomainError::BadRequest("invalid_date".to_string()))?;
    Ok(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()))
}

fn render_orders_csv(items: &[MerchantOrderReportItem]) -> String {
    let mut lines = Vec::with_capacity(items.len() + 1);
    lines.push("order_id,demand_id,status,total_amount,paid_amount,refund_amount,created_at".to_string());
    for item in items {
        let demand_id = item.demand_id.map(|v| v.to_string()).unwrap_or_default();
        lines.push(format!(
            "{},{},{},{:.2},{:.2},{:.2},{}",
            item.id,
            demand_id,
            item.status,
            item.total_amount,
            item.paid_amount,
            item.refund_amount,
            item.created_at
        ));
    }
    lines.join("\n")
}
