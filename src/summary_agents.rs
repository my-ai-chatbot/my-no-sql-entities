// Description of AI
// PARTITION_KEY - made as string concatenation as '{inventory_id}|{llm_model_id}'
// ROW_KEY - id of record

use std::collections::HashMap;

use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

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
    pub fn generate_partition_key(inventory_id: &str, llm_model_id: &str) -> String {
        format!("{inventory_id}_{llm_model_id}")
    }

    pub fn generate_row_key(id: String) -> String {
        id
    }

    pub fn get_inventory_id(&self) -> &str {
        let mut parts = self.partition_key.split('|');
        parts.next().unwrap()
    }

    pub fn get_llm_model_id(&self) -> &str {
        let mut parts = self.partition_key.split('|');
        parts.next();
        parts.next().unwrap()
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
