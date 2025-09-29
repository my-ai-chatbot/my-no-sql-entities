use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// PartitionKey - {company|brand}
// RowKey - {id}

#[my_no_sql_entity("car-sales-location")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarSalesLocationMyNoSqlEntity {
    pub title: String,
    pub address: String,
    pub sale_phone: Option<String>,
    pub service_phone: Option<String>,
    pub working_hours: Vec<WorkingHoursModel>,
    pub image: String,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkingHoursModel {
    pub dow: String,
    pub info: String,
}
