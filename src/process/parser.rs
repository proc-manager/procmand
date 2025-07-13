use crate::common::models;
use std::fs;
use std::io::BufReader;

pub fn parse(json_file_path: &str) -> Result<models::ProcessConfig, Box<dyn std::error::Error>> {
    let file = fs::File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let config: models::ProcessConfig = serde_json::from_reader(reader)?;
    Ok(config)
}
