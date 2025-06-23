use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("stt-config")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SttConfigMyNoSqlEntity {
    pub dest_id: String,
}

impl SttConfigMyNoSqlEntity {
    pub fn get_profile_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_lang_id(&self) -> &str {
        &self.partition_key
    }
}
