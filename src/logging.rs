use crate::Logger;
use crate::Result;
use wapc_guest::host_call;
use wascc_codec::logging::*;
use wascc_codec::serialize;

/// The reserved capability ID for the logging functionality
pub const CAPID_LOGGING: &str = "wascc:logging";

pub struct DefaultLogger {}

impl DefaultLogger {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DefaultLogger {
    fn default() -> Self {
        DefaultLogger {}
    }
}

impl Logger for DefaultLogger {
    fn log(&self, actor: &str, level: usize, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: level,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }
    fn error(&self, actor: &str, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: 1, // should this be a constant??
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn warn(&self, actor: &str, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: 2, // should this be a constant??
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn info(&self, actor: &str, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: 3, // should this be a constant??
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn debug(&self, actor: &str, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: 4, // should this be a constant??
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }

    fn trace(&self, actor: &str, body: &str) -> Result<()> {
        let l = WriteLogRequest {
            level: 5, // should this be a constant??
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }
}
