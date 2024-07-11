use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub management_url: String,
    pub rtfm: bool
}

pub fn load_config() -> Option<Config> {
    match fs::create_dir_all(rvepp_common::CONFIGURATION_BASE_PATH) {
        Ok(_) => {
            println!("Configuration path did not exist and was created");
        }
        Err(err) => {
            println!("Error creating the configuration due to: {err}");

            return None;
        }
    };

    let json_string = fs::read_to_string(rvepp_common::CONFIGURATION_BASE_PATH.to_string() + rvepp_common::CONFIGURATION_FILE_NAME)
        .expect("Should have been able to read the file");

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