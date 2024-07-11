use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub management_url: String,
    pub rtfm: bool
}

fn json_to_config(json_string: String) -> Option<Config> {
    return match serde_json::from_str::<Config>(&json_string) {
        Ok(config) => {
            Some(config)
        }
        Err(err) => {
            println!("Error creating the configuration due to: {err}");

            None
        }
    };
}

fn validate_path() -> Option<bool> {
    return match fs::create_dir_all(rvepp_common::CONFIGURATION_BASE_PATH) {
        Ok(_) => {
            println!("Configuration path did not exist and was created");

            Some(true)
        }
        Err(err) => {
            println!("Error creating the configuration due to: {err}");

            None
        }
    };
}

fn create_default_config() -> Config {
    return Config {
        management_url: "https://rvepp.io".to_string(),
        rtfm: true
    };
}

fn create_default_file(full_path: String) -> Option<Config> {
    let config = create_default_config();

    let json_string = serde_json::to_string(&config).expect("Failure in serializing the configuration");

    return match fs::write(full_path.clone(), json_string) {
        Ok(_) => Some(config),
        Err(err) => {
            println!("Error writing default config to {full_path} due to: {err}");

            None
        }
    };
}

pub fn load_config() -> Option<Config> {
    if validate_path() == None {
        return None;
    }

    let full_path = rvepp_common::CONFIGURATION_BASE_PATH.to_string() + rvepp_common::CONFIGURATION_FILE_NAME;

    if !fs::metadata(full_path.clone()).is_ok() {
        return create_default_file(full_path);
    }

    return match fs::read_to_string(&full_path) {
        Ok(json_string) => json_to_config(json_string),
        Err(err) => {
            println!("Failed to read configuration file: {err}");

            None
        }
    }
}