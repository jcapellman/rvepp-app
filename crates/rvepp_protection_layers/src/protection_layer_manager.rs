use rvepp_configuration;
use crate::ProtectionLayer;
use crate::rtfm::Rtfm;

pub struct ProtectionLayerManager {
}

impl ProtectionLayerManager {
    pub fn initialize(&self, config: rvepp_configuration::Config) -> Option<bool> {
        if config.rtfm {
            println!("Initializing RTFM...");

            let mut real_time_monitoring = Rtfm { };

            real_time_monitoring.initialize();

            println!("RTFM Initialized...");
        }

        return Some(true);
    }
}