use log::{Metadata, Record};
use crate::logging::DefaultLogger;
use crate::Logger;

struct Logger {
    l: DefaultLogger
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            l: DefaultLogger{}
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.l.log("actor", record.metadata().level(), "body");
            //println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};

static LOGGER: Logger = Logger{l: DefaultLogger{}};

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

#[cfg(test)]
mod test {
    use log::info;
    use super::*;
    use crate::Logger;

    static LOGGER: Logger = Logger{l: DefaultLogger{}};
    #[test]
    fn logger() {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
        info!("brian");
    }
}
