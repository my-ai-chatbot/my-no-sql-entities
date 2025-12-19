use chat_bot_common::languages::Language;
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

    pub silence_millis: Option<usize>,
}

impl VadSettingsMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "vs";

    pub const ROW_KEY: &'static str = "global";

    pub const DEFAULT_SILENCE_MS: usize = 1000;

    pub fn get_lang(&self) -> Language {
        Language::from_str(&self.row_key)
    }

    pub fn generate_row_key(lang: Language) -> &'static str {
        lang.as_str()
    }

    pub fn get_silence_ms(&self) -> usize {
        self.silence_millis.unwrap_or(Self::DEFAULT_SILENCE_MS)
    }
}

impl Default for VadSettingsMyNoSqlEntity {
    fn default() -> Self {
        Self {
            partition_key: Self::PARTITION_KEY.to_string(),
            row_key: Self::ROW_KEY.to_string(),
            time_stamp: Default::default(),
            frame_size: 512,
            threshold: 0.2,
            min_speech_duration_ms: 250,
            speech_pad_ms: 1000,
            silence_millis: Some(1000),
        }
    }
}
