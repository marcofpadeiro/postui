use std::{error::Error, fs, path::Path};

use super::model::Request;

pub fn parse_file(file_path: &String) -> Result<Request, Box<dyn Error>> {
    let file: String = fs::read_to_string(file_path)?;
    let result = toml::from_str(&file)?;

    Ok(result)
}

pub fn write_into_file<P: AsRef<Path>>(request: &Request, file_path: P) -> Result<(), Box<dyn Error>> {
    let contents = toml::to_string_pretty(request)?;

    let path = file_path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?; 
    }

    fs::write(path, contents)?;

    Ok(())
}
