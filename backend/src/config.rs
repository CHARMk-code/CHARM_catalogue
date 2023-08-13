use std::{env, fs};

use serde::Deserialize;

use log::{error, info};
// TODO: Write information about this configuration in documentation
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub upload_path: String,
    pub storage_path: String,
    pub password_salt: String,
    pub cors_allowed_origin: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "".to_string(),
            upload_path: "upload/".to_string(),
            storage_path: "storage/".to_string(),
            password_salt: "NOT A GOOD SALT".to_string(),
            cors_allowed_origin: "localhost".to_string(),
        }
    }
}

pub fn get_config(path: &str) -> Config {
    let config: Config = fs::read_to_string(path)
        .map_err(|_| error!("Failed reading config file. Applying default instead"))
        .and_then(|string| {
            toml::from_str(&string).map_err(|err| {
                error!(
                    "Failed parsing config file with error: {} \nApplying default instead",
                    err.message()
                )
            })
        })
        .unwrap_or(Config::default());
        
    // override the created config with new values from env variables when one exists
    let new_config = Config {
        database_url: replace_with_env_if_exist("DATABASE_URL", config.database_url),
        upload_path: replace_with_env_if_exist("UPLOAD_PATH", config.upload_path),
        storage_path: replace_with_env_if_exist("STORAGE_PATH", config.storage_path),
        password_salt: replace_with_env_if_exist("PASSWORD_SALT", config.password_salt),
        cors_allowed_origin: replace_with_env_if_exist(
            "CORS_ALLOWED_ORIGIN",
            config.cors_allowed_origin,
        ),
    };
    
    info!("Config set to: {:?}", new_config);

    new_config

}

pub fn replace_with_env_if_exist<T: From<String>>(name: &str, config_value: T) -> T {
    match env::var(name) {
        Ok(env_value) => {
            info!("environment value {} replacing config value", name);
            env_value.into()
        }
        Err(..) => config_value,
    }
}
