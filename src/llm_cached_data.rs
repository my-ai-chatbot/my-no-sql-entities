//Delete me at some point of time
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("llm-cached-data")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmCacheDataMyNoSqlEntity {
    pub value: String,
}
