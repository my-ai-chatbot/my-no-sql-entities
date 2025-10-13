use chat_bot_common::{inventory_type::InventoryType, llm_model_type::ChatBotLlmModel};
use serde::*;

use crate::llm::LlmGeneralSettings;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("chat-tests")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatTestMyNoSqlEntity {
    pub description: String,
    pub chat_history: Vec<ChatHistoryEvent>,
    pub result_to_assert: String,
    pub disabled: bool,
    #[serde(default)]
    pub time_zone: String,
    #[serde(default)]
    pub time_offset: String,
    #[serde(default)]
    pub country_by_ip: String,
    #[serde(default)]
    pub sys_prompt: String,
    #[serde(default)]
    pub additional_instruction: String,
    #[serde(default)]
    pub llm_model: ChatBotLlmModel,
    #[serde(default)]
    pub assert_llm_settings: LlmGeneralSettings,
}

impl ChatTestMyNoSqlEntity {
    pub fn generate_partition_key(inventory_type: InventoryType) -> &'static str {
        inventory_type.as_str()
    }

    pub fn get_inventory_type(&self) -> InventoryType {
        InventoryType::from_str(&self.partition_key)
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
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
