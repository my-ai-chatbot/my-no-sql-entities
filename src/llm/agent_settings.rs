use std::collections::HashMap;

use chat_bot_common::{
    Gpt5ReasoningEffort, Gpt5Verbosity,
    inventory_type::{self, InventoryType},
    llm_model_type::ChatBotLlmModel,
};
use serde::*;

use crate::llm::LlmGeneralSettings;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("agents-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentSettingsMyNoSqlEntity {
    pub prompts: Option<HashMap<String, String>>,

    pub prompts_voice: Option<HashMap<String, String>>,

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

    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_settings: Option<LlmGeneralSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<LlmGeneralSettings>,

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

    pub fn generate_partition_key(inventory_type: InventoryType) -> String {
        inventory_type.to_string()
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

    pub fn get_text_llm_settings(&self) -> LlmGeneralSettings {
        if let Some(result) = self.text_settings.clone() {
            return result;
        }

        let from_partition_key = self.get_from_partition_key();
        LlmGeneralSettings {
            llm_model_id: from_partition_key.1.unwrap_or_default(),
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
