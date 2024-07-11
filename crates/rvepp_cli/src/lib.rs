use std::env;

pub fn run() {
    println!("rvepp {}", env!("CARGO_PKG_VERSION"));
    println!("(C) 2024 Jarred Capellman");
    println!("Source code is available on https://github.com/jcapellman/rvepp");

    let args: Vec<String> = env::args().collect();

    let config = rvepp_configuration::Config {
        management_url: "https://rvepp.io".to_string(),
        rtfm: true
    };

    let protection_manager = rvepp_protection_layers::protection_layer_manager::ProtectionLayerManager { };

    let initialize_result = protection_manager.initialize(config);

    match initialize_result {
        Some(true) => println!("Initialized the Protection Manager"),
        Some(false) => println!("Failed to Initialize the Protection Manager"),
        None => println!("Error occurred")
    }
}