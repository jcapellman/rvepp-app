#[cfg(target_os = "linux")]
fn main() {
    rvepp_cli::run();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("rvepp only supports Linux - shutting down")
}