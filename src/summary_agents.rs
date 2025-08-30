use std::collections::HashMap;

use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//PARTITION_KEY - inventory

#[my_no_sql_entity("summary-agents")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryAgentMyNoSqlEntity {
    pub tech_prompts: HashMap<String, String>,
    pub prompts: HashMap<String, String>,

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

impl SummaryAgentMyNoSqlEntity {
    pub fn get_model(&self) -> &str {
        &self.row_key
    }
}
