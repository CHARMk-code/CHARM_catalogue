use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: Option<String>,
    pub upload_path: String,
    pub storage_path: String,
    pub password_salt: Vec<u8>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: None,
            upload_path: "upload/".to_string(),
            storage_path: "storage/".to_string(),
            password_salt: b"NOT A GOOD SALT".to_vec(),
        }
    }
}

pub fn read_config(path: &str) -> Result<Config, std::io::Error> {
    let config_string = fs::read_to_string(path)?;

    match toml::from_str(config_string.as_str()) {
        Ok(config) => Ok(config),
        Err(error) => {
            println!(
                "Trying to parse config, received the following error: {:?}.\n Applying default instead",
                error
            );
            Ok(Config::default())
        }
    }
}
