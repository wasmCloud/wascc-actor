use crate::Logger;
use crate::Result;
use log::{Metadata, Record};
use wapc_guest::host_call;
use wascc_codec::logging::*;
use wascc_codec::serialize;

/// The reserved capability ID for the logging functionality
pub const CAPID_LOGGING: &str = "wascc:logging";

const NONE: usize = 0;
const ERROR: usize = 1;
const WARN: usize = 2;
const INFO: usize = 3;
const DEBUG: usize = 4;
const TRACE: usize = 5;

pub struct AutomaticLogger {}

impl AutomaticLogger {
    pub fn new() -> Self {
        Self::default()
    }
    fn _log(&self, req: WriteLogRequest) {
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(req).unwrap());
    }
}

impl Default for AutomaticLogger {
    fn default() -> Self {
        AutomaticLogger {}
    }
}

impl log::Log for AutomaticLogger {
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
impl Logger for AutomaticLogger {
    fn log(&self, level: usize, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: level,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }
    fn error(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: ERROR,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn warn(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: WARN,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn info(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: INFO,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn debug(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: DEBUG,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn trace(&self, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: TRACE,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }
}
