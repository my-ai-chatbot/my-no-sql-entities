use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("translations")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslationsMyNoSqlEntity {
    pub translation: String,
}

impl TranslationsMyNoSqlEntity {
    pub fn generate_partition_key(profile_id: &str, language_id: &str) -> String {
        format!("{}|{}", profile_id, language_id)
    }

    pub fn get_profile_id(&self) -> &str {
        let index = self.partition_key.find('|').unwrap();

        &self.partition_key[..index]
    }

    pub fn get_lang(&self) -> &str {
        let index = self.partition_key.find('|').unwrap();

        &self.partition_key[index + 1..]
    }

    pub fn get_translation_key(&self) -> &str {
        &self.row_key
    }
}
