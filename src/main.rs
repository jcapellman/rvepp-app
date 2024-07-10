#[cfg(target_os = "linux")]
fn main() {
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

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("rvepp only supports Linux - shutting down")
}