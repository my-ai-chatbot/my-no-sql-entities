use std::collections::HashMap;

use chat_bot_common::{
    LlmAgentOtherSettings,
    inventory_type::{self, InventoryType},
    llm_model_type::ChatBotLlmModel,
};
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("agents-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentSettingsMyNoSqlEntity {
    pub prompts: Option<HashMap<String, String>>,

    pub prompts_voice: Option<HashMap<String, String>>,

    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub last_edited: i64,
    pub disable_think: Option<bool>,
    pub verbosity: Option<String>,
    pub reasoning_effort: Option<String>,
    pub who: String,
}

impl AgentSettingsMyNoSqlEntity {
    pub fn get_from_partition_key(&self) -> (InventoryType, ChatBotLlmModel) {
        let mut split = self.partition_key.split('|');

        let inventory_type = split.next().unwrap_or_default();
        let llm_model = split.next().unwrap_or_default();

        let inventory_type =
            inventory_type::InventoryType::try_from_str(inventory_type).unwrap_or_default();
        let llm_model = ChatBotLlmModel::try_from_str(llm_model).unwrap_or_default();
        (inventory_type, llm_model)
    }

    pub fn generate_partition_key(inventory_type: InventoryType, model: ChatBotLlmModel) -> String {
        format!("{}|{}", inventory_type.as_str(), model.as_str())
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

impl LlmAgentOtherSettings for AgentSettingsMyNoSqlEntity {
    fn get_temperature(&self) -> Option<f64> {
        self.temperature
    }

    fn get_top_p(&self) -> Option<f64> {
        self.top_p
    }

    fn get_top_k(&self) -> Option<i64> {
        self.top_k
    }

    fn get_n(&self) -> Option<i64> {
        self.n
    }

    fn get_presence_penalty(&self) -> Option<f64> {
        self.presence_penalty
    }

    fn get_frequency_penalty(&self) -> Option<f64> {
        self.frequency_penalty
    }

    fn get_disable_think(&self) -> Option<bool> {
        self.disable_think
    }
}
