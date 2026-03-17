use log::{info, error};

pub fn setup_logger() {
    env_logger::init();
    info!("Logger initialized.");
}

pub fn log_error(message: &str) {
    error!("Error: {}", message);
}

