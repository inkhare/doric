use crate::backend::LoggerConfig;
use crate::encode::parser::Message;
use crate::runtime::append::Appender;

#[derive(Debug)]
pub struct Extend {
    pub lc: LoggerConfig,
}

impl Extend {
    pub fn new(conf: LoggerConfig) -> Extend {
        Extend { lc: conf }
    }
}

impl Appender for Extend {
    fn append(&self, msg: &Message) -> Result<(), String> {
        println!("fgfgh log line {:?}", msg);
        Ok(())
    }
}
