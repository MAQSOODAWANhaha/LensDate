use std::collections::HashMap;

use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::dto::pagination::{normalize_pagination, Paged};
use crate::dto::photographers::{PhotographerListItem, PhotographerListQuery};
use crate::entity::{photographers, user_profiles, users};
use crate::errors::ServiceResult;
use crate::state::AppState;

pub async fn list_photographers(
    state: &AppState,
    query: PhotographerListQuery,
) -> ServiceResult<Paged<PhotographerListItem>> {
    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let offset = (page - 1) * page_size;

    let mut q = photographers::Entity::find();

    if query.status.as_deref().is_some_and(|status| status != "approved") {
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }
    q = q.filter(photographers::Column::Status.eq("approved"));

    if let Some(r#type) = &query.r#type {
        q = q.filter(photographers::Column::Type.eq(r#type));
    }

    if let Some(city_id) = query.city_id {
        q = q.filter(photographers::Column::CityId.eq(city_id));
    }

    if let Some(keyword) = query.keyword.as_deref() {
        let keyword = keyword.trim();
        if !keyword.is_empty() {
            let mut user_ids: Vec<i64> = users::Entity::find()
                .filter(users::Column::Phone.contains(keyword))
                .all(&state.orm)
                .await?
                .into_iter()
                .map(|u| u.id)
                .collect();

            let mut profile_user_ids: Vec<i64> = user_profiles::Entity::find()
                .filter(user_profiles::Column::Nickname.contains(keyword))
                .all(&state.orm)
                .await?
                .into_iter()
                .map(|p| p.user_id)
                .collect();

            user_ids.append(&mut profile_user_ids);
            user_ids.sort_unstable();
            user_ids.dedup();

            if user_ids.is_empty() {
                return Ok(Paged::new(Vec::new(), 0, page, page_size));
            }
            q = q.filter(photographers::Column::UserId.is_in(user_ids));
        }
    }

    let total = q.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Paged::new(Vec::new(), 0, page, page_size));
    }

    let rows = q
        .order_by_desc(photographers::Column::RatingAvg)
        .order_by_desc(photographers::Column::CompletedOrders)
        .order_by_desc(photographers::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    if rows.is_empty() {
        return Ok(Paged::new(Vec::new(), total, page, page_size));
    }

    let user_ids: Vec<i64> = rows.iter().map(|p| p.user_id).collect();
    let profile_rows = user_profiles::Entity::find()
        .filter(user_profiles::Column::UserId.is_in(user_ids.clone()))
        .all(&state.orm)
        .await?;

    let mut profile_map: HashMap<i64, (Option<String>, Option<String>)> = HashMap::new();
    for row in profile_rows {
        profile_map.insert(row.user_id, (row.nickname.clone(), row.avatar_url.clone()));
    }

    let items = rows
        .into_iter()
        .map(|p| {
            let (nickname, avatar_url) = profile_map
                .get(&p.user_id)
                .cloned()
                .unwrap_or((None, None));
            PhotographerListItem {
                id: p.id,
                user_id: p.user_id,
                r#type: p.r#type,
                status: p.status,
                city_id: p.city_id,
                service_area: p.service_area,
                nickname,
                avatar_url,
                rating_avg: decimal_to_f64(p.rating_avg),
                completed_orders: p.completed_orders,
            }
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    use std::str::FromStr;
    f64::from_str(&v.to_string()).unwrap_or(0.0)
}
