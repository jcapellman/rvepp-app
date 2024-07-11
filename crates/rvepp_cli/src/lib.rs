use std::env;

pub fn run() {
    println!("rvepp {}", env!("CARGO_PKG_VERSION"));
    println!("(C) 2024 Jarred Capellman");
    println!("Source code is available on https://github.com/jcapellman/rvepp");

    let args: Vec<String> = env::args().collect();

    let internal_vars = rvepp_cmdl_args::parse_args(args);

    let config = match rvepp_configuration::load_config(internal_vars) {
        None => {
            panic!("Failed to load configuration - exiting");
        }
        Some(val) => val
    };

    let protection_manager = rvepp_protection_layers::protection_layer_manager::ProtectionLayerManager { };

    let initialize_result = protection_manager.initialize(config);

    match initialize_result {
        Some(true) => println!("Initialized the Protection Manager"),
        Some(false) => println!("Failed to Initialize the Protection Manager"),
        None => println!("Error occurred")
    }
}