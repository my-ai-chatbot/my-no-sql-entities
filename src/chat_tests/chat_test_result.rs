use chat_bot_common::{inventory_type::InventoryType, llm_model_type::ChatBotLlmModel};
use serde::*;

use crate::llm::LlmGeneralSettings;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("chat-test-results")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatTestResultsMyNoSqlEntity {
    pub active_process: Option<ActiveProcess>,
    pub last_result: Option<LastResult>,
    pub status: Option<ChatTestStatus>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ChatTestStatus {
    InQueue,
    InProgress,
    Success,
    Fail,
}

impl ChatTestStatus {
    pub fn is_in_progress(&self) -> bool {
        match self {
            Self::InProgress => true,
            _ => false,
        }
    }
}

impl ChatTestResultsMyNoSqlEntity {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveProcess {
    pub started: i64,
    pub process_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastResult {
    pub ok: bool,
    pub message: String,
    #[serde(default)]
    pub assert_result: String,
}
