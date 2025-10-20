use chat_bot_common::languages::Language;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("stt-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SttProfileMyNoSqlEntity {
    pub primary: String,
    pub secondary: Option<String>,
    pub chat_id: Option<String>,
    pub version: Option<String>,
}

impl SttProfileMyNoSqlEntity {
    pub fn get_profile_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_lang_id(&self) -> Language {
        Language::from_str(&self.row_key)
    }
}
