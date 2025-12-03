use chat_bot_common::*;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("tenants")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantMyNoSqlEntity {
    pub domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_system_prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stt_profile: Option<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_llm_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_every_round_trip: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_trim_history_round_trip: Option<usize>,

    #[serde(default)]
    pub white_listed_countries: Vec<String>,

    #[serde(default)]
    pub ip_white_list: Vec<String>,

    #[serde(default)]
    pub tts_route: TtsOption,
}

impl TenantMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "t";

    pub fn get_tenant_id(&self) -> &str {
        &self.row_key
    }

    pub fn get_summary_every_round_trip(&self) -> usize {
        self.summary_every_round_trip.unwrap_or(3)
    }

    pub fn get_request_trim_history_round_trip(&self) -> usize {
        self.request_trim_history_round_trip.unwrap_or(10)
    }
}
