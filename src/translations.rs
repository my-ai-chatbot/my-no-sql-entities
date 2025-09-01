use chat_bot_common::{inventory_type::InventoryType, languages::Language};
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("translations")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslationsMyNoSqlEntity {
    pub translation: String,
}

impl TranslationsMyNoSqlEntity {
    pub fn generate_partition_key(
        inventory_type: InventoryType,
        profile_id: &str,
        language_id: Language,
    ) -> String {
        format!(
            "{}|{}|{}",
            inventory_type.as_str(),
            profile_id,
            language_id.as_str()
        )
    }

    pub fn get_from_partition_key(&self) -> (InventoryType, &str, Language) {
        let mut index = self.partition_key.split('|');

        let first = index.next().unwrap();
        let second = index.next().unwrap();
        let third = index.next();

        if let Some(third) = third {
            return (
                InventoryType::from_str(first),
                second,
                Language::from_str(third),
            );
        }

        (InventoryType::default(), first, Language::from_str(second))
    }

    pub fn get_translation_key(&self) -> &str {
        &self.row_key
    }
}
