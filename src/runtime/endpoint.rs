use crate::backend::LoggerConfig;
use crate::config;
use crate::encode::parser::Message;
use crate::runtime::append::AppendBuilder;
use crate::runtime::append::Appender;
use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time};

pub struct Endpoint {
    #[allow(dead_code)]
    interval: u64,
    rx: Arc<Mutex<SegQueue<Message>>>,
    appender: Box<dyn Appender>,
}

pub struct EndpointBuilder {
    interval: u64,
}

impl EndpointBuilder {
    pub fn new() -> EndpointBuilder {
        EndpointBuilder { interval: 10 }
    }

    pub fn build(&mut self, conf: &config::Config, rx: Arc<Mutex<SegQueue<Message>>>) {
        let lc = LoggerConfig {
            path: conf.path.clone(),
            max_size: conf.max_size * 1024 * 1024,
            max_segments: conf.max_segments,
            log_type: conf.log_type.clone(),
        };

        let mut interval = self.interval;
        if conf.interval != 0 {
            interval = conf.interval;
        }

        let mut ep = Endpoint {
            interval: interval,
            rx: rx,
            appender: AppendBuilder::build(lc),
        };

        ep.run()
    }
}

impl Endpoint {
    pub fn run(&mut self) {
        let delay = time::Duration::from_millis(self.interval);
        let mut cycle = 0;
        loop {
            if !self.rx.lock().unwrap().is_empty() {
                let msg = self.rx.lock().unwrap().pop().unwrap();
                match self.appender.append(&msg) {
                    Ok(_v) => {}
                    Err(e) => {
                        println!("error = {:?}", e);
                    }
                };
            } else {
                cycle += 1;
            }

            if cycle == 10 {
                thread::sleep(delay);
                cycle = 0;
            }
        }
    }
}
