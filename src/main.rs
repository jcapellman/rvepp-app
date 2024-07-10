#[cfg(target_os = "linux")]
use rvepp_protection_layers::{ProtectionLayer, rtfm};

#[cfg(target_os = "linux")]
fn main() {
    let mut real_time_file_monitoring = rtfm::Rtfm { };

    real_time_file_monitoring.initialize();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("rvepp only supports Linux - shutting down")
}