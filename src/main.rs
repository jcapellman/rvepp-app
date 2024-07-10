#[cfg(target_os = "linux")]
use rvepp_protection_layers::{RTFM};

#[cfg(target_os = "linux")]
fn main() {
    let rtfm = RTFM { };

    rtfm.initialize();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("rvepp only supports Linux...exiting...")
}