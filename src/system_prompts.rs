use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("system-prompts")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPromptMyNoSqlEntity {
    pub value: String,
    pub last_edited: String,
    pub who: String,
}

impl SystemPromptMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "s";

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
