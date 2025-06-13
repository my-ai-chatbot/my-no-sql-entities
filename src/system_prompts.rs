use std::collections::HashMap;

use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("system-prompts")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPromptMyNoSqlEntity {
    pub value: String,

    pub prompts: Option<HashMap<String, String>>,

    pub temperature: Option<u64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i64>,
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub last_edited: i64,
    pub who: String,
}

impl SystemPromptMyNoSqlEntity {
    pub fn get_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_model(&self) -> &str {
        &self.row_key
    }
}
