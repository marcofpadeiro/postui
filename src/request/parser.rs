use std::{error::Error, fs};

use super::model::Request;

pub fn parse_file(file_path: &String) -> Result<Request, Box<dyn Error>> {
    let file: String = fs::read_to_string(file_path)?;
    let result = toml::from_str(&file)?;

    Ok(result)
}
