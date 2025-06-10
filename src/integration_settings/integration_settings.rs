use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();
use super::*;

#[enum_of_my_no_sql_entity("integration-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IntegrationSettingsMyNoSqlEntity {
    SparkPostSettings(SparkPostSettingsModel),
    OpenApiSettings(OpenApiSettingsModel),
}
