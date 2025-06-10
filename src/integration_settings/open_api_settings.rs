use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key: "p", row_key:"open-api")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenApiSettingsModel {
    pub api_key: String,
}
