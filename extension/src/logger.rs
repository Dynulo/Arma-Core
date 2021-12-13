use arma_rs::rv_callback;
use log::{Level, LevelFilter, Metadata, Record};

use crate::rv_send_callback;

struct ArmaLogger;

impl log::Log for ArmaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            rv_callback!(
                "dynulo_log",
                "core",
                format!("{}", record.level()).to_uppercase(),
                format!("{}", record.args())
            );
        }
    }

    fn flush(&self) {}
}
static LOGGER: ArmaLogger = ArmaLogger;

pub fn init() {
    if let Err(e) = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)) {
        error!("failed to initialize logger: {}", e);
    }
}
