//Delete me at some point of time
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// Partition Key - FullPath without query-string
// Row Key - Scroll Position

const POS_QUANT: u64 = 500; // Scroll position quantization, e.g., 500px

#[my_no_sql_entity("dyn-content-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DynContentCacheMyNoSqlEntity {
    pub value: String,
}

impl DynContentCacheMyNoSqlEntity {
    pub fn new(scheme_domain: &str, path: &str, position: u64, value: String) -> Self {
        let full_path = if path.starts_with('\\') {
            format!("{}{}", scheme_domain, path)
        } else {
            format!("{}/{}", scheme_domain, path)
        };

        Self {
            partition_key: full_path,
            row_key: round_scroll_position(position).to_string(),
            time_stamp: Default::default(),
            value,
        }
    }
}

pub fn round_scroll_position(position: u64) -> u64 {
    position / POS_QUANT * POS_QUANT
}

#[cfg(test)]
mod test {

    #[test]
    fn test_round_position() {
        assert_eq!(super::round_scroll_position(0), 0);
        assert_eq!(super::round_scroll_position(100), 0);
        assert_eq!(super::round_scroll_position(400), 0);
        assert_eq!(super::round_scroll_position(500), 500);
        assert_eq!(super::round_scroll_position(800), 500);
    }
}
