use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn read_text(path: &str) -> Result<String> {
    let mut text = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut text)?;
    Ok(text)
}
