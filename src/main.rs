mod process;
mod common;

use log::{info, LevelFilter};
use env_logger::Builder;
use process::parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    info!("reading the json config");

    let config = parser::parse("process.json")?;

    println!("config: {:?}", config);

    info!("done reading json config");

    Ok(())
}
