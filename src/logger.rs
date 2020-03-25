use log::{Metadata, Record};
use crate::logging::DefaultLogger;
use crate::Logger;

pub struct AutomaticLogger {
    pub l: DefaultLogger
}

impl AutomaticLogger {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AutomaticLogger {
    fn default() -> Self {
        AutomaticLogger {
            l: DefaultLogger{}
        }
    }
}

impl log::Log for AutomaticLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {

        if self.enabled(record.metadata()) {
            match self.l.log( record.level() as _, &format!("{}",record.args())) {
                Ok(r) => crate::console_log("logger:log ok"),
                Err(r) => crate::console_log("logger:log not ok{}"),
            }
        }
    }

    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};

static LOGGER: AutomaticLogger = AutomaticLogger{l: DefaultLogger{}};

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

#[cfg(test)]
mod test {
    use log::info;
    use super::*;
    use crate::Logger;

    static LOGGER: AutomaticLogger = AutomaticLogger{l: DefaultLogger{}};
    #[test]
    fn logger() {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
        info!("brian");
    }
}
