use crate::Result;
use log::{Metadata, Record};
use wapc_guest::host_call;
use wascc_codec::logging::*;
use wascc_codec::serialize;

/// The reserved capability ID for the logging functionality
pub const CAPID_LOGGING: &str = "wascc:logging";

const ERROR: u32 = 1;
const WARN: u32 = 2;
const INFO: u32 = 3;
const DEBUG: u32 = 4;
const TRACE: u32 = 5;

use std::sync::{Arc, RwLock};

lazy_static! {
    pub static ref LOG_BINDING: Arc<RwLock<String>> =
        { Arc::new(RwLock::new("default".to_string())) };
}

static LOGGER: AutomaticLoggerHostBinding = AutomaticLoggerHostBinding {};

lazy_static! {
    pub static ref ENSURE_LOGGER: bool = {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        true
    };
}

/// A host binding for the wascc:logging capability
pub struct AutomaticLoggerHostBinding {}

impl Default for AutomaticLoggerHostBinding {
    fn default() -> Self {
        AutomaticLoggerHostBinding {}
    }
}

/// Sets the current binding for the logger. Note that because the logger
/// is statically available in order to support Rust's `log` macros,
/// the _msot recent binding_ you set is the one that will
/// be active for your logging macros. You can't maintain two different logger
/// bindings that send to two different host bindings, you must instead toggle
/// between them.
pub fn host(binding: &str) -> AutomaticLoggerHostBinding {
    *LOG_BINDING.write().unwrap() = binding.to_string();
    AutomaticLoggerHostBinding {}
}

/// Resets the current logger binding name to the default.
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
    fn _log(&self, req: WriteLogRequest) {
        let _ = host_call(
            &LOG_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(req).unwrap(),
        );
    }

    /// Write a log entry on the host
    pub fn log(&self, level: u32, body: &str) -> Result<()> {
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

    /// Write a log entry at the error level. You should instead use the `error!` macro
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

    /// Write a log entry at the warn level. You should instead use the `warn!` macro
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

    /// Write a log entry at the info level. You should instead use the `info!` macro
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

    /// Write a log entry at the debug level. You should instead use the `debug!` macro
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

    /// Write a log entry at the trace level. You should instead use the `trace!` macro
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
