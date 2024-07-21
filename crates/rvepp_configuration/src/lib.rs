use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use rvepp_cmdl_args::InternalVars;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub management_url: String,
    pub model_elf_file_name: String,
    pub rtfm: bool
}

fn json_to_config(json_string: String) -> Config {
    return match serde_json::from_str::<Config>(&json_string) {
        Ok(config) => {
            config
        }
        Err(err) => {
            println!("Error creating the configuration - using defaults, error: {err}");

            create_default_config()
        }
    };
}

fn validate_path(path: &String) -> Option<bool> {
    if Path::new(path).exists() {
        println!("Configuration path exists {}, attempting to read file...", path);

        return Some(true);
    }

    return match fs::create_dir_all(path) {
        Ok(_) => {
            println!("Configuration path did not exist and was created {}", path);

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
        management_url: rvepp_common::constants::CONFIGURATION_URL.to_string(),
        rtfm: true,
        model_elf_file_name: rvepp_common::constants::CONFIGURATION_MODEL_ELF_FILE_NAME.to_string()
    };
}

fn create_default_file(full_path: String) -> Config {
    let config = create_default_config();

    let json_string = serde_json::to_string(&config).expect("Failure in serializing the configuration");

    return match fs::write(full_path.clone(), json_string) {
        Ok(_) => config,
        Err(err) => {
            println!("Error writing default config to {full_path} due to: {err}");

            return config;
        }
    };
}

pub fn load_config(internal_vars: InternalVars) -> Config {
    if validate_path(&internal_vars.config_path) == None {
        return create_default_config();
    }

    let full_path = internal_vars.config_path + &*internal_vars.config_filename;

    if !fs::metadata(full_path.clone()).is_ok() {
        return create_default_file(full_path);
    }

    return match fs::read_to_string(&full_path) {
        Ok(json_string) => json_to_config(json_string),
        Err(err) => {
            println!("Failed to read configuration file using defaults, error: {err}");

            return create_default_config();
        }
    }
}