use doric::config;
use log::{info, error};
use std::{thread, time};

fn main() {
    let conf = config::Config {
        path: "./log".to_string(),
        max_size: 2,
        max_segments: 3,
        interval: 10,
        level: doric::Info,
        log_type: doric::File,
    };

    config::init_config(&conf);

    error!("log test{:?}", 1234);

    for i in 1..4 {
        let _handle = thread::spawn(move || {
            error!("log test{:?} hello, moto", i);
        });
    }

    for i in 10..14 {
        info!("log test{:?}", i);
    }

    info!("info log test{:?}", 777);
    error!("error log test{:?}", 777);

    let delay = time::Duration::from_millis(2 * 1000);
    thread::sleep(delay);
}

#[test]
fn simple_log() {
    let conf = config::Config {
        path: "./log".to_string(),
        max_size: 2,
        max_segments: 3,
        interval: 10,
        level: doric::Info,
        log_type: doric::File,
    };

    config::init_config(&conf);
    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    error!("error log test{:?}", 9090);
}
