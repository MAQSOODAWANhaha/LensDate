use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NotificationItem {
    pub id: i64,
    pub r#type: String,
    pub title: String,
    pub content: Option<String>,
    pub read_at: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct ListNotificationsQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub read_status: Option<String>,
}

#[derive(Serialize)]
pub struct NotificationSummary {
    pub unread_count: u64,
}

#[derive(Serialize)]
pub struct MarkAllReadResp {
    pub updated: u64,
}
