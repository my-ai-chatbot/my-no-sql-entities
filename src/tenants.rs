use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("tenants")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantMyNoSqlEntity {
    pub domains: Vec<String>,
    pub languages: Option<Vec<String>>,
    pub default_language: Option<String>,
    pub audio_enabled: Option<String>,
    pub enabled: bool,
}

impl TenantMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "t";

    pub fn get_tenant_id(&self) -> &str {
        &self.row_key
    }
}
