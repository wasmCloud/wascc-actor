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
            actor: actor.to_string(),
            level: level,
            body: body.to_string(),
        };
        let _ = host_call(CAPID_LOGGING, OP_LOG, &serialize(l)?);
        Ok(())
    }
}
