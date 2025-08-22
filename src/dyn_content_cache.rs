//Delete me at some point of time
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// Partition Key - FullPath without query-string
// Row Key - Scroll Position

#[my_no_sql_entity("dyn-content-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DynContentCacheMyNoSqlEntity {
    pub value: String,
}
