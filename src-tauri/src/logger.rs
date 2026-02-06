use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::env;
use std::path::PathBuf;
use chrono::Local;

fn get_log_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("diskofflaner_history.log");
    path
}

pub fn log_activity(message: &str) {
    let path = get_log_path();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!("[{}] {}\n", timestamp, message);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open log file");

    let _ = file.write_all(log_entry.as_bytes());
}

pub fn get_logs() -> Vec<String> {
    let path = get_log_path();
    if !path.exists() {
        return vec![];
    }

    read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn clear_logs() {
    let path = get_log_path();
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}
