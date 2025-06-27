mod process;
mod common;

use process::parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = parser::parse("process.yaml")?;

    p.pprint();

    Ok(())

}
