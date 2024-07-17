use std::env;
use std::thread;
use std::time::Duration;

use log::{LevelFilter};
use log::{info, error};

pub fn run() {
    log::set_logger(&rvepp_common::logger::LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).expect("Failed to initialize logger");

    info!("---------------------------------------");
    info!("RVEPP {}", env!("CARGO_PKG_VERSION"));
    info!("(C) 2024 Jarred Capellman");
    info!("https://github.com/jcapellman/rvepp-app");
    info!("---------------------------------------");

    let args: Vec<String> = env::args().collect();

    let internal_vars = rvepp_cmdl_args::parse_args(args);

    let config = rvepp_configuration::load_config(internal_vars);

    info!("Starting the Protection Layers...");

    let protection_manager = rvepp_protection_layers::protection_layer_manager::ProtectionLayerManager { };

    let initialize_result = protection_manager.initialize(config);

    match initialize_result {
        Some(true) => info!("Initialized the Protection Manager"),
        Some(false) => error!("Failed to Initialize the Protection Manager"),
        None => error!("Error occurred")
    }

    loop {
        thread::sleep(Duration::from_millis(100));
    }
}