use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub r#type: String,
    pub order_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ConversationResp {
    pub id: i64,
    pub r#type: String,
    pub order_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct ListConversationsQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub order_id: Option<i64>,
}
