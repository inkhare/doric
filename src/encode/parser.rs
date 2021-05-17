use log::Level;

#[derive(Debug)]
pub struct Message {
    pub level: Level,
    pub args: String,
}

#[derive(Debug)]
pub struct Parser {
    mode: i32,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { mode: 0 }
    }

    pub fn encode(&self, record: &log::Record) -> Result<Message, String> {
        let m = Message {
            level: record.level(),
            args: record.args().to_string(),
        };
        Ok(m)
    }
}
