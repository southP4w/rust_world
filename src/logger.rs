use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Logger struct for managing log files
pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(log_dir: &str, log_file_name: &str) -> io::Result<Self> {
        if !Path::new(log_dir).exists() {
            std::fs::create_dir(log_dir)?;
        }
        let file_path = format!("{}/{}", log_dir, log_file_name);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(Self { file })
    }

    pub fn log(&mut self, message: &str) -> io::Result<()> {
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        writeln!(self.file, "{} {}", timestamp, message)?;
        self.file.flush()
    }
}
