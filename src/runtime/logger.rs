use super::endpoint;
use crate::config;
use crate::encode::parser;
use crate::encode::parser::Message;
use arc_swap::ArcSwap;
use crossbeam_queue::SegQueue;
use log::{LevelFilter, Metadata};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
pub struct Handler {
    h: parser::Parser,
    tx: Arc<Mutex<SegQueue<Message>>>,
}

impl Handler {
    pub fn log(&self, record: &log::Record) -> Result<(), String> {
        let line = self.h.encode(record).unwrap();
        self.tx.lock().unwrap().push(line);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RawLogger {
    level: LevelFilter,
    attacher: Handler,
}

impl RawLogger {
    pub fn new(conf: &config::Config) -> RawLogger {
        let q = SegQueue::new();
        let lockq = Arc::new(Mutex::new(q));
        let lq = Arc::clone(&lockq);

        let sub_conf = conf.clone();

        let _handle = thread::spawn(move || {
            endpoint::EndpointBuilder::new().build(&sub_conf, lq);
        });

        let h = Handler {
            h: parser::Parser::new(),
            tx: lockq,
        };

        RawLogger {
            level: conf.level,
            attacher: h,
        }
    }

    pub fn max_log_level(&self) -> LevelFilter {
        self.level
    }
}

#[derive(Debug)]
pub struct Logger(Arc<ArcSwap<RawLogger>>);

impl Logger {
    pub fn new(conf: &config::Config) -> Logger {
        let raw_logger = RawLogger::new(conf);
        Logger(Arc::new(ArcSwap::new(Arc::new(raw_logger))))
    }

    pub fn max_log_level(&self) -> LevelFilter {
        self.0.load().max_log_level()
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let shared = self.0.load();
        match shared.attacher.log(record) {
            Ok(_v) => {}
            Err(e) => {
                println!("error = {:?}", e);
            }
        }
    }

    fn flush(&self) {}
}
