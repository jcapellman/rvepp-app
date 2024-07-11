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

pub fn load_config() -> Option<Config> {
    if validate_path() == None {
        return None;
    }

    let json_string = fs::read_to_string(rvepp_common::CONFIGURATION_BASE_PATH.to_string() + rvepp_common::CONFIGURATION_FILE_NAME)
        .expect("Should have been able to read the file");

    return json_to_config(json_string);
}