use rvepp_configuration;
use crate::ProtectionLayer;
use crate::rtfm::Rtfm;

pub struct ProtectionLayerManager {
}

impl ProtectionLayerManager {
    pub fn initialize(&self, config: rvepp_configuration::Config) -> Option<bool> {
        if config.rtfm {
            let mut real_time_monitoring = Rtfm { };

            real_time_monitoring.initialize();
        }

        return Some(true);
    }
}