extern crate rflex;
use std::fs;
use std::io::{Error, ErrorKind::NotFound};
use std::path::Path;

fn generate_parse() -> Result<(), Error> {
    let input = Path::new("src").join("parse.l");
    let input = input.to_str().ok_or(Error::from(NotFound))?;
    let input_meta = fs::metadata(input)?;
    let output = Path::new("src").join("parse.rs");
    let output_meta = fs::metadata(&output);

    if let Ok(output_meta) = output_meta {
        if output_meta.modified()? >= input_meta.modified()? {
            println!("{:?} is up to date", output);
            return Ok(());
        }
    }
    println!("Generating {:?}...", output);
    rflex::process(input.to_string())?;
    Ok(())
}

fn main() -> Result<(), Error> {
    generate_parse()
}
