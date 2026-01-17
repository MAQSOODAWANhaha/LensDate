use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub conversation_id: i64,
    pub content: Option<String>,
    pub msg_type: String,
}

#[derive(Serialize)]
pub struct MessageResp {
    pub id: i64,
    pub conversation_id: i64,
}

#[derive(Deserialize)]
pub struct ListMessagesQuery {
    pub conversation_id: i64,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct MessageItem {
    pub id: i64,
    pub conversation_id: i64,
    pub sender_id: i64,
    pub content: Option<String>,
    pub msg_type: String,
    pub sent_at: String,
}
