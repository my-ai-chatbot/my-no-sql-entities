use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// PartitionKey - {company|brand}
// RowKey - {id}

#[my_no_sql_entity("car-inventory")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarInventoryMyNoSqlEntity {
    pub model: String,
    pub price: f64,
    pub year: i64,
    pub mileage: f64,
    pub drive_train: Option<String>,
    pub efficiency_label: Option<String>,
    pub engine_type: Option<String>,
    pub fuel: Option<String>,
    pub transmission: Option<String>,
    pub options: Vec<String>,

    pub image: String,
    pub interior_images: Vec<String>,
}

impl CarInventoryMyNoSqlEntity {
    pub fn generate_partition_key(company_id: &str, car_brand: &str) -> String {
        format!("{}|{}", company_id, car_brand)
    }

    pub fn get_from_partition_key<'s>(&'s self) -> CrateInventoryFromPartitionKey<'s> {
        let mut split = self.partition_key.split('|');

        let company_id = split.next().unwrap_or_default();
        let car_brand = split.next().unwrap_or_default();

        CrateInventoryFromPartitionKey {
            company_id,
            car_brand,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}

pub struct CrateInventoryFromPartitionKey<'s> {
    pub company_id: &'s str,
    pub car_brand: &'s str,
}
