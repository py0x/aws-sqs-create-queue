use std::fs;
use std::path::Path;
use serde::Deserialize;
use anyhow::{Context, Result};
use toml;

#[derive(Deserialize, Debug)]
pub struct QueueAttr {
    pub name: String,
    pub visibility_timeout: u32,
    pub message_retention_period: u32,
    pub message_max_receive_count: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct QueueConfig {
    pub queue: QueueAttr,
    pub dead_letter_queue: Option<QueueAttr>,
}

#[derive(Deserialize, Debug)]
struct Config {
    queues: Vec<QueueConfig>,
}

pub fn load_queue_config<P: AsRef<Path>>(path: P) -> Result<Vec<QueueConfig>> {
    let file_path = path.as_ref();

    let conf_content = fs::read_to_string(&path)
        .context(format!("Failed to read file: {}", file_path.display()))?;

    let config: Config = toml::from_str(&conf_content)
        .context(format!("Failed to parse config: {}", file_path.display()))?;

    Ok(config.queues)
}