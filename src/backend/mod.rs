use crate::encode::encoder;

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub path: String,
    pub max_size: u64,
    pub max_segments: usize,
    pub log_type: encoder::EncoderType,
}

pub mod console;
pub mod extend;
pub mod material;
