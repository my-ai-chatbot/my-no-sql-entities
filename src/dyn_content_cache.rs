//Delete me at some point of time
use serde::*;
use service_sdk::{
    my_no_sql_sdk::abstractions::Timestamp, rust_extensions::date_time::DateTimeAsMicroseconds,
};

service_sdk::macros::use_my_no_sql_entity!();

// Partition Key - FullPath without query-string
// Row Key - Scroll Position

const POS_QUANT: u64 = 100; // Scroll position quantization, e.g., 500px

#[my_no_sql_entity("dyn-content-cache")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DynContentCacheMyNoSqlEntity {
    pub value: String,
    pub expires: Timestamp,
}

impl DynContentCacheMyNoSqlEntity {
    pub fn new(
        scheme_domain_path: String,
        position: u64,
        value: String,
        mut now: DateTimeAsMicroseconds,
    ) -> Self {
        now.add_hours(24);
        Self {
            partition_key: scheme_domain_path,
            row_key: round_scroll_position(position).to_string(),
            time_stamp: Default::default(),
            value,
            expires: now.into(),
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
        assert_eq!(super::round_scroll_position(50), 0);
        assert_eq!(super::round_scroll_position(100), 100);
        assert_eq!(super::round_scroll_position(400), 400);
        assert_eq!(super::round_scroll_position(410), 400);
        assert_eq!(super::round_scroll_position(500), 500);
        assert_eq!(super::round_scroll_position(800), 800);
    }
}
