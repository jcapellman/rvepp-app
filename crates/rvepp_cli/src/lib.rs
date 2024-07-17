use std::env;
use std::thread;
use std::time::Duration;

pub fn run() {
    println!("---------------------------------------");
    println!("RVEPP {}", env!("CARGO_PKG_VERSION"));
    println!("(C) 2024 Jarred Capellman");
    println!("https://github.com/jcapellman/rvepp-app");
    println!("---------------------------------------");

    let args: Vec<String> = env::args().collect();

    let internal_vars = rvepp_cmdl_args::parse_args(args);

    let config = rvepp_configuration::load_config(internal_vars);

    println!("Starting the Protection Layers...");

    let protection_manager = rvepp_protection_layers::protection_layer_manager::ProtectionLayerManager { };

    let initialize_result = protection_manager.initialize(config);

    match initialize_result {
        Some(true) => println!("Initialized the Protection Manager"),
        Some(false) => println!("Failed to Initialize the Protection Manager"),
        None => println!("Error occurred")
    }

    loop {
        thread::sleep(Duration::from_millis(100));
    }
}