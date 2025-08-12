//Delete me at some point of time
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("widget-clients")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetClientMyNoSqlEntity {
    pub content: String,
    pub size: usize,
}

impl WidgetClientMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "wc";

    pub fn get_app_id(&self) -> &str {
        &self.row_key
    }
}
