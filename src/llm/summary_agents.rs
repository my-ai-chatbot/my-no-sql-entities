// Description for AI
// PARTITION_KEY - made as string concatenation as '{inventory_id}|{llm_model_id}'
// ROW_KEY - id of record
// tech_prompts - map of tech prompts used to generate technical summary which is a json file. Key is a language of the prompt;
// prompts - map of prompts used to generate human readable text summary. Key is a language of the prompt;

use std::collections::HashMap;

use chat_bot_common::{inventory_type::InventoryType, llm_model_type::*, *};
use serde::*;

use super::LlmGeneralSettings;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("summary-agents")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryAgentMyNoSqlEntity {
    pub tech_prompts: HashMap<String, String>,
    pub prompts: HashMap<String, String>,

    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    pub last_edited: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    pub who: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_label: Option<String>,

    #[serde(default)]
    pub llm_model: ChatBotLlmModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_settings: Option<LlmGeneralSettings>,
}

impl SummaryAgentMyNoSqlEntity {
    pub fn generate_partition_key(inventory_type: InventoryType) -> String {
        inventory_type.to_string()
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

    pub fn get_llm_settings(&self) -> LlmGeneralSettings {
        if let Some(llm_settings) = self.llm_settings.clone() {
            return llm_settings;
        }
        LlmGeneralSettings {
            temperature: self.temperature,
            top_p: self.top_p,
            n: self.n,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            think: self.think,
            verbosity: match self.verbosity.as_deref() {
                Some(value) => Gpt5Verbosity::try_from_str(value),
                None => None,
            },
            reasoning_effort: match self.reasoning_effort.as_deref() {
                Some(value) => Gpt5ReasoningEffort::try_from_str(value),
                None => None,
            },
            mcp_label: self.mcp_label.clone(),
            prompt_id: self.prompt_id.clone(),
            prompt_version: self.prompt_version.clone(),
        }
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
