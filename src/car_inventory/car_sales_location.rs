use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// PartitionKey - {company|brand}
// RowKey - {id}

#[my_no_sql_entity("car-sales-location")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarSalesLocationMyNoSqlEntity {
    pub title: String,
    pub address: String,
    pub city: String,
    pub sale_phone: Option<String>,
    pub service_phone: Option<String>,
    pub working_hours: Vec<WorkingHoursModel>,
    pub image: String,
    pub latitude: String,
    pub longitude: String,
}

impl CarSalesLocationMyNoSqlEntity {
    pub fn generate_partition_key(company_id: &str, car_brand: &str) -> String {
        format!("{}|{}", company_id, car_brand)
    }

    pub fn get_from_partition_key<'s>(&'s self) -> CarSalesLocationFromPartitionKey<'s> {
        let mut split = self.partition_key.split('|');

        let company_id = split.next().unwrap_or_default();
        let car_brand = split.next().unwrap_or_default();

        CarSalesLocationFromPartitionKey {
            company_id,
            car_brand,
        }
    }

    pub fn get_id(&self) -> &str {
        self.row_key.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkingHoursModel {
    pub dow: String,
    pub info: String,
}

pub struct CarSalesLocationFromPartitionKey<'s> {
    pub company_id: &'s str,
    pub car_brand: &'s str,
}
