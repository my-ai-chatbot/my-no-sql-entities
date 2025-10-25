// Description for AI
// PARTITION_KEY - made as string concatenation as '{inventory_id}|{llm_model_id}'
// ROW_KEY - id of record
// tech_prompts - map of tech prompts used to generate technical summary which is a json file. Key is a language of the prompt;
// prompts - map of prompts used to generate human readable text summary. Key is a language of the prompt;

use std::collections::HashMap;

use chat_bot_common::{inventory_type::InventoryType, languages::Language, llm_model_type::*};
use serde::*;

use super::LlmGeneralSettings;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("summary-agents")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryAgentMyNoSqlEntity {
    #[serde(default)]
    pub last_edited: i64,
    #[serde(default)]
    pub who: String,

    #[serde(default)]
    pub summary_llm_settings: HashMap<Language, LlmGeneralSettings>,

    #[serde(default)]
    pub tech_summary_llm_settings: HashMap<Language, LlmGeneralSettings>,
}

impl SummaryAgentMyNoSqlEntity {
    pub fn generate_partition_key(inventory_type: InventoryType) -> &'static str {
        inventory_type.as_str()
    }

    pub fn generate_row_key(prompt_id: &str) -> &str {
        prompt_id
    }

    pub fn get_inventory_type_and_llm_model(
        &self,
    ) -> Result<(InventoryType, Option<ChatBotLlmModel>), String> {
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

        let llm_model = if let Some(part) = parts.next() {
            match ChatBotLlmModel::try_from_str(part) {
                Some(llm_model) => Some(llm_model),
                None => Some(ChatBotLlmModel::default()),
            }
        } else {
            None
        };

        Ok((inventory_type, llm_model))
    }

    pub fn get_prompt_id(&self) -> &str {
        &self.row_key
    }
}

/*
impl LlmAgentGenericSettings for SummaryAgentMyNoSqlEntity {
    fn get_temperature(&self) -> Option<f64> {
        self.temperature
    }

    fn get_top_p(&self) -> Option<f64> {
        self.top_p
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

    fn get_think(&self) -> bool {
        self.think.unwrap_or_default()
    }

    fn get_reasoning_effort(&self) -> Option<chat_bot_common::Gpt5ReasoningEffort> {
        let value = self.reasoning_effort.as_deref()?;
        chat_bot_common::Gpt5ReasoningEffort::try_from_str(value)
    }

    fn get_verbosity(&self) -> Option<chat_bot_common::Gpt5Verbosity> {
        let value = self.verbosity.as_deref()?;
        chat_bot_common::Gpt5Verbosity::try_from_str(value)
    }
}
 */
