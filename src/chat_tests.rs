use chat_bot_common::inventory_type::InventoryType;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("chat-tests")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatTestMyNoSqlEntity {
    pub description: String,
    pub chat_history: Vec<ChatHistoryItem>,
    pub result_to_assert: String,
    pub disabled: bool,
}

impl ChatTestMyNoSqlEntity {
    pub fn generate_partition_key(inventory_type: InventoryType) -> String {
        inventory_type.as_str().to_string()
    }

    pub fn get_inventory_type(&self) -> InventoryType {
        InventoryType::from_str(&self.partition_key)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatHistoryItem {
    pub role: ChatHistoryRole,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatHistoryRole {
    User,
    Assistant,
}
