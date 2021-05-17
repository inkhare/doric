use crate::encode::encoder;
use crate::runtime::logger;
use log::LevelFilter;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
    pub max_size: u64,
    pub interval: u64,
    pub max_segments: usize,
    pub level: LevelFilter,
    pub log_type: encoder::EncoderType,
}

/// Initial a logger from config.
///
/// # Examples
/// ```
/// let conf = config::Config {
///     path: "./log".to_string(),
///     max_size: 2,
///     max_segments: 3,
///     interval: 10,
///     level: doric::Info,
///     log_type: doric::File,
/// };
/// config::init_config(&conf);
/// error!("error log test{:?}", 9090);
/// ```
pub fn init_config(cfg: &Config) {
    let logger = logger::Logger::new(cfg);
    log::set_max_level(cfg.level);

    match log::set_boxed_logger(Box::new(logger)).map(|()| true) {
        Ok(_v) => {}
        Err(e) => {
            println!("error = {:?}", e);
        }
    };
}
