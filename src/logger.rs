use crate::Result;
use log::{Metadata, Record};
use wapc_guest::host_call;
use wascc_codec::logging::*;
use wascc_codec::serialize;

/// The reserved capability ID for the logging functionality
pub const CAPID_LOGGING: &str = "wascc:logging";

const ERROR: usize = 1;
const WARN: usize = 2;
const INFO: usize = 3;
const DEBUG: usize = 4;
const TRACE: usize = 5;

use std::sync::{Arc, RwLock};

lazy_static! {
    pub static ref LOG_BINDING: Arc<RwLock<String>> =
        { Arc::new(RwLock::new("default".to_string())) };
}

static LOGGER: AutomaticLoggerHostBinding = AutomaticLoggerHostBinding {};

lazy_static! {
    pub static ref DUMMY: bool = {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        true
    };
}

pub struct AutomaticLoggerHostBinding {}

impl Default for AutomaticLoggerHostBinding {
    fn default() -> Self {
        AutomaticLoggerHostBinding {}
    }
}

pub fn host(binding: &str) -> AutomaticLoggerHostBinding {
    *LOG_BINDING.write().unwrap() = binding.to_string();
    AutomaticLoggerHostBinding {}
}

pub fn default() -> AutomaticLoggerHostBinding {
    *LOG_BINDING.write().unwrap() = "default".to_string();
    AutomaticLoggerHostBinding {}
}

impl log::Log for AutomaticLoggerHostBinding {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let l = WriteLogRequest {
                level: record.level() as _,
                body: format!("{}", record.args()),
            };
            self._log(l)
        }
    }

    fn flush(&self) {}
}

impl AutomaticLoggerHostBinding {
    pub fn new() -> Self {
        Self::default()
    }
    fn _log(&self, req: WriteLogRequest) {
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(req).unwrap(),
        );
    }

    pub fn log(&self, level: usize, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: level,
            body: body.to_string(),
        };
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }
    pub fn error(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: ERROR,
            body: body.to_string(),
        };
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }

    pub fn warn(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: WARN,
            body: body.to_string(),
        };
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }

    pub fn info(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: INFO,
            body: body.to_string(),
        };
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }

    pub fn debug(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: DEBUG,
            body: body.to_string(),
        };
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }

    pub fn trace(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: TRACE,
            body: body.to_string(),
        };
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }
}
