use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();
use super::SparkPostSettingsModel;

#[enum_of_my_no_sql_entity("product-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProductSettings {
    SparkPostSettings(SparkPostSettingsModel),
}
