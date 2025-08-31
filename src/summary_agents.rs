// Description for AI
// PARTITION_KEY - made as string concatenation as '{inventory_id}|{llm_model_id}'
// ROW_KEY - id of record
// tech_prompts - map of tech prompts used to generate technical summary which is a json file. Key is a language of the prompt;
// prompts - map of prompts used to generate human readable text summary. Key is a language of the prompt;

use std::collections::HashMap;

use chat_bot_common::{
    LlmAgentOtherSettings, inventory_type::InventoryType, llm_model_type::ChatBotLlmModel,
};
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("summary-agents")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryAgentMyNoSqlEntity {
    pub tech_prompts: HashMap<String, String>,
    pub prompts: HashMap<String, String>,

    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i64>,
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub last_edited: i64,
    pub disable_think: Option<bool>,
    pub who: String,
}

impl SummaryAgentMyNoSqlEntity {
    pub fn generate_partition_key(
        inventory_type: InventoryType,
        llm_model_id: ChatBotLlmModel,
    ) -> String {
        format!("{}|{}", inventory_type.as_str(), llm_model_id.as_str())
    }

    pub fn generate_row_key(prompt_id: &str) -> &str {
        prompt_id
    }

    pub fn get_inventory_type_and_llm_model(
        &self,
    ) -> Result<(InventoryType, ChatBotLlmModel), String> {
        let mut parts = self.partition_key.split('|');
        let inventory_type = match InventoryType::try_from_str(parts.next().unwrap()) {
            Some(inventory_type) => inventory_type,
            None => {
                return Err(format!(
                    "Invalid inventory type in partition key: {}",
                    self.partition_key
                ));
            }
        };

        let llm_model = match ChatBotLlmModel::try_from_str(parts.next().unwrap()) {
            Some(llm_model) => llm_model,
            None => {
                return Err(format!(
                    "Invalid LlmModel type in partition key: {}",
                    self.partition_key
                ));
            }
        };

        Ok((inventory_type, llm_model))
    }

    pub fn get_prompt_id(&self) -> &str {
        &self.row_key
    }
}

impl LlmAgentOtherSettings for SummaryAgentMyNoSqlEntity {
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

    fn disable_think(&self) -> Option<bool> {
        self.disable_think
    }
}
