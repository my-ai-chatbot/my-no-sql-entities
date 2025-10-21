use chat_bot_common::llm_model_type::ChatBotLlmModel;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LlmGeneralSettings {
    pub llm_model_id: ChatBotLlmModel,
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<chat_bot_common::Gpt5Verbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<chat_bot_common::Gpt5ReasoningEffort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_label: Option<String>,
}

/*
pub trait LlmAgentGenericSettings {
    fn get_temperature(&self) -> Option<f64>;
    fn get_top_p(&self) -> Option<f64>;
    fn get_n(&self) -> Option<i64>;
    fn get_presence_penalty(&self) -> Option<f64>;
    fn get_frequency_penalty(&self) -> Option<f64>;
    fn get_think(&self) -> bool;
    fn get_reasoning_effort(&self) -> Option<Gpt5ReasoningEffort>;
    fn get_verbosity(&self) -> Option<Gpt5Verbosity>;
}

impl LlmAgentGenericSettings for LlmGeneralSettings {
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
