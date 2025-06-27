mod process;
mod common;

use process::parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = parser::parse("process.json")?;

    println!("config: {:?}", config);

    Ok(())
}
