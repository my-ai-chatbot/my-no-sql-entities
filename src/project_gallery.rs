use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

//PartitionKey = "tenant_data_id";
//RowKey = "project_name";

#[my_no_sql_entity("project-gallery")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectGalleryMyNoSqlEntity {
    pub images: Vec<String>,
}
