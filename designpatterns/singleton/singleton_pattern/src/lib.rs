use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

pub struct Logger {
    log_level: String,
}

impl Logger {
    pub fn log(&self, message: &str) {
        println!("[{}] {}", self.log_level, message);
    }

    pub fn set_log_level(&mut self, level: String) {
        self.log_level = level;
    }
}

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger {
        log_level: String::from("INFO"),
    }));
}

pub fn get_logger() -> Arc<Mutex<Logger>> {
    Arc::clone(&INSTANCE)
}

