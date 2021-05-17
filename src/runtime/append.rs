use crate::backend::console::Console;
use crate::backend::extend::Extend;
use crate::backend::material::Material;
use crate::backend::LoggerConfig;
use crate::encode::encoder::EncoderType;
use crate::encode::parser::Message;
use std::fmt;

pub trait Appender: fmt::Debug + Send + Sync + 'static {
    fn append(&self, msg: &Message) -> Result<(), String>;
}

#[derive(Debug)]
pub struct AppendBuilder {
    pub id: i32,
}

impl AppendBuilder {
    pub fn build(conf: LoggerConfig) -> Box<dyn Appender> {
        match conf.log_type {
            EncoderType::Console => {
                return Box::new(Console::new(conf));
            }
            EncoderType::File => {
                return Box::new(Material::new(conf));
            }
            EncoderType::EncoderExtend(ref _t) => {
                return Box::new(Extend::new(conf));
            }
        }
    }
}
