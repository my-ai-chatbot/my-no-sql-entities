use std::collections::HashMap;

use chat_bot_common::{LlmAgentOtherSettings, llm_model_type::ChatBotLlmModel};
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("system-prompts")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPromptMyNoSqlEntity {
    pub prompts: Option<HashMap<String, String>>,

    pub prompts_voice: Option<HashMap<String, String>>,

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

impl SystemPromptMyNoSqlEntity {
    pub fn generate_row_key(model: ChatBotLlmModel) -> &'static str {
        model.as_str()
    }

    pub fn get_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_model(&self) -> ChatBotLlmModel {
        match ChatBotLlmModel::try_from_str(self.row_key.as_str()) {
            Some(model) => model,
            None => ChatBotLlmModel::default(),
        }
    }
}

impl LlmAgentOtherSettings for SystemPromptMyNoSqlEntity {
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
