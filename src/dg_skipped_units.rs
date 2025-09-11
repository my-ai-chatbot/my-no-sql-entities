use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("dg-skipped-units")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DgSkippedUnitMyNoSqlEntity {
    pub city: String,
    pub code: String,
}

impl DgSkippedUnitMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "dg-s";

    pub fn get_project_name(&self) -> &str {
        &self.row_key
    }
}
