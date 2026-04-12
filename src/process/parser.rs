use crate::common::models;
use anyhow::{Context, Result};
use std::fs;
use std::io::BufReader;

pub fn parse(json_file_path: &str) -> Result<models::ProcessConfig> {
    let file = fs::File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let config: models::ProcessConfig = serde_json::from_reader(reader).context("error reading process config")?;
    Ok(config)
}
