/*
 * Provides a simple set of logging utilities for the tool
 */
pub fn log_info(msg: &str) {
    println!("\x1b[32m[INFO]\x1b[0m {}", msg);
}

pub fn log_warn(msg: &str) {
    println!("\x1b[33m[WARN]\x1b[0m {}", msg);
}

pub fn log_debug(msg: &str) {
    println!("\x1b[34m[DEBUG]\x1b[0m {}", msg);
}

pub fn log_error(msg: &str) {
    eprintln!("\x1b[31m[ERROR]\x1b[0m {}", msg);
}
