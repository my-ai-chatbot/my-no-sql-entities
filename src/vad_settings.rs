//Delete me at some point of time
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("vad-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VadSettingsMyNoSqlEntity {
    pub frame_size: usize,
    pub threshold: f32,
    pub min_speech_duration_ms: usize,
    pub speech_pad_ms: usize,
}

impl VadSettingsMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "vs";
    pub const ROW_KEY: &'static str = "global";
}
