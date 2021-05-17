use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EncoderType {
    Console,
    File,
    EncoderExtend(String),
}

pub trait Encoder: fmt::Debug + Send + Sync + 'static {
    fn encode(&self, record: &log::Record) -> Result<(), String>;
}
