use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("badges")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgesMyNoSqlEntity {
    pub value: String,
    pub tp: String,
}
impl BadgesMyNoSqlEntity {
    pub const PARTITION_KEY_ADMIN: &'static str = "a";
    pub const ROW_KEY_MISSED_DG_UNITS: &'static str = "mdu";
}
