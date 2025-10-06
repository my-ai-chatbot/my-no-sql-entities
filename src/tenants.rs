use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("tenants")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantMyNoSqlEntity {
    pub domains: Vec<String>,
    pub languages: Option<Vec<String>>,
    pub default_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enabled: Option<bool>,
    pub system_prompt_id: Option<String>,
    pub summary_system_prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations_profile: Option<String>,
    pub inventory_type: Option<String>,
    pub stt_profile: Option<String>,
    pub enabled: bool,
    pub llm_model: Option<String>,
    pub summary_llm_model: Option<String>,
    pub app_id: Option<String>,
    pub dynamic_content: Option<bool>,
    pub authorized_paths: Option<Vec<String>>,
    pub summary_every_round_trip: Option<usize>,
    pub request_trim_history: Option<usize>,
}

impl TenantMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "t";

    pub fn get_tenant_id(&self) -> &str {
        &self.row_key
    }

    pub fn get_summary_every_round_trip(&self) -> usize {
        self.summary_every_round_trip.unwrap_or(3)
    }

    pub fn get_request_trim_history(&self) -> usize {
        self.request_trim_history.unwrap_or(10)
    }
}
