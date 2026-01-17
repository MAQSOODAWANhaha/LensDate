use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PhotographerListQuery {
    pub keyword: Option<String>,
    pub city_id: Option<i64>,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct PhotographerListItem {
    pub id: i64,
    pub user_id: i64,
    pub r#type: String,
    pub status: String,
    pub city_id: Option<i64>,
    pub service_area: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub rating_avg: f64,
    pub completed_orders: i32,
}
