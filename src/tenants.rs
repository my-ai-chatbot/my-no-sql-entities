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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations_profile: Option<String>,
    pub enabled: bool,
}

impl TenantMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "t";

    pub fn get_tenant_id(&self) -> &str {
        &self.row_key
    }
}
