use log::{Record, Level, Metadata};

pub struct RVEPPLogger;

impl log::Log for RVEPPLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub static LOGGER: RVEPPLogger = RVEPPLogger;