use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("chat-tests")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatTestMyNoSqlEntity {
    pub chat_history: Vec<ChatHistoryEvent>,
    pub result_to_assert: String,
    pub tenant_id: String,
    pub disabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChatHistoryRole {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatHistoryEvent {
    pub role: ChatHistoryRole,
    pub text: String,
}
