use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("dg-projects")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DgProjectMyNoSqlEntity {
    pub title: String,
    pub description: String,
    pub image: String,
    pub project_type: String,
    pub project_url: String,
    pub amenities: Vec<String>,
}

impl DgProjectMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "dg";
    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
