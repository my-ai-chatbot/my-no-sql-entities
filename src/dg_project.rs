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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_names_to_map: Option<Vec<u8>>,
}

impl DgProjectMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "dg";
    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
