use std::{fs::{self, File}, io::ErrorKind, path::PathBuf};

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {

}

#[allow(dead_code)]
fn ensure_dir_exists(path: &PathBuf) -> std::io::Result<()>{
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    
    Ok(())
}

#[allow(dead_code)]
fn get_config_dir_path() -> std::io::Result<PathBuf> {
    match ProjectDirs::from("com", "postui", "postui") {
        Some(proj_dirs) => {
            let config_dir = proj_dirs.config_dir().to_path_buf();
            ensure_dir_exists(&config_dir)?;

            Ok(config_dir)
        }
        None => {
            Err(std::io::Error::new(ErrorKind::NotFound, "Could not find platform-specific config directory"))
        }
    }
}

#[allow(dead_code)]
pub fn get_requests_dir_path() -> std::io::Result<PathBuf> {
    let requests_path = get_config_dir_path()?.join("requests");
    ensure_dir_exists(&requests_path)?;
    Ok(requests_path)
}

#[allow(dead_code)]
pub fn get_environments_dir_path() -> std::io::Result<PathBuf> {
    let environments_path = get_config_dir_path()?.join("environments");
    ensure_dir_exists(&environments_path)?;
    Ok(environments_path)
}

#[allow(dead_code)]
pub fn get_config_file_path() -> std::io::Result<PathBuf> {
    let config_file_path = get_config_dir_path()?.join("config.toml");
    if !config_file_path.exists() {
        File::create(&config_file_path)?;
    }
    Ok(config_file_path)
}

#[allow(dead_code)]
fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = get_config_file_path()?;
    let contents = fs::read_to_string(path)?;

    Ok(toml::from_str(&contents)?)
}
