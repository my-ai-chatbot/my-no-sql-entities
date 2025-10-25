use std::collections::HashMap;

use chat_bot_common::{
    inventory_type::{self, InventoryType},
    languages::Language,
    llm_model_type::ChatBotLlmModel,
};
use serde::*;

use crate::llm::LlmGeneralSettings;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("agents-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentSettingsMyNoSqlEntity {
    pub last_edited: i64,
    #[serde(default)]
    pub text_settings: HashMap<Language, LlmGeneralSettings>,
    #[serde(default)]
    pub voice_settings: HashMap<Language, LlmGeneralSettings>,
    #[serde(default)]
    pub who: String,
}

impl AgentSettingsMyNoSqlEntity {
    pub fn get_from_partition_key(&self) -> (InventoryType, Option<ChatBotLlmModel>) {
        let mut split = self.partition_key.split('|');

        let inventory_type = split.next().unwrap_or_default();
        let llm_model = split.next();

        let llm_model = match llm_model {
            Some(llm_model) => Some(ChatBotLlmModel::try_from_str(llm_model).unwrap_or_default()),
            None => None,
        };

        let inventory_type =
            inventory_type::InventoryType::try_from_str(inventory_type).unwrap_or_default();

        (inventory_type, llm_model)
    }

    pub fn generate_partition_key(inventory_type: InventoryType) -> &'static str {
        inventory_type.as_str()
    }

    pub fn generate_row_key(id: &str) -> &str {
        id
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }

    pub fn get_version(&self) -> usize {
        let mut result = 0;

        for itm in self.partition_key.chars() {
            if itm == '|' {
                result += 1;
            }
        }

        result
    }
}
