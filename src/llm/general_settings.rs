use chat_bot_common::llm_model_type::ChatBotLlmModel;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LlmGeneralSettings {
    //Added as string - to make sure - we do not recompile chat-bot-api - since it does not use it but need it
    #[serde(default)]
    pub llm_model_id: String,
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
    pub prompt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_url: Option<String>,
}

impl LlmGeneralSettings {
    pub fn get_llm_id(&self) -> ChatBotLlmModel {
        if let Some(llm_model) = ChatBotLlmModel::try_from_str(&self.llm_model_id) {
            return llm_model;
        }

        for itm in ChatBotLlmModel::ALL {
            if format!("{:?}", itm) == self.llm_model_id {
                return *itm;
            }
        }

        ChatBotLlmModel::default()
    }
}
