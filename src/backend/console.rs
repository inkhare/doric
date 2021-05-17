use crate::backend::LoggerConfig;
use crate::encode::parser::Message;
use crate::runtime::append::Appender;

#[derive(Debug)]
pub struct Console {
    pub lc: LoggerConfig,
}

impl Console {
    pub fn new(conf: LoggerConfig) -> Console {
        Console { lc: conf }
    }

    fn output(&self, msg: &Message) {
        let data = format!(
            "{} {} {}",
            chrono::offset::Local::now().to_string(),
            msg.level.to_string(),
            msg.args
        );
        println!("{}", data);
    }
}

impl Appender for Console {
    fn append(&self, msg: &Message) -> Result<(), String> {
        self.output(msg);
        Ok(())
    }
}
