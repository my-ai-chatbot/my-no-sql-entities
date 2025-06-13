//Delete me at some point of time
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("llm-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmSettingsMyNoSqlEntity {
    pub temperature: Option<u64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i64>,
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
}

impl LlmSettingsMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "l";

    pub fn get_profile_id(&self) -> &str {
        &self.row_key
    }
}
