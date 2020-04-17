// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Errors
//!
//! This module contains is the default host logger implementation.
//! wapc_guest provides the host shim.

use crate::Result;
use codec::logging::*;
use codec::serialize;
use log::{Metadata, Record};
use std::sync::{Arc, RwLock};
use wapc_guest::host_call;
use wascc_codec as codec;

/// The reserved capability ID for the logging functionality
pub const CAPID_LOGGING: &str = "wascc:logging";

const ERROR: u32 = 1;
const WARN: u32 = 2;
const INFO: u32 = 3;
const DEBUG: u32 = 4;
const TRACE: u32 = 5;

lazy_static! {
    static ref CURRENT_BINDING: Arc<RwLock<String>> =
        { Arc::new(RwLock::new("default".to_string())) };
}

static LOGGER: AutomaticLoggerHostBinding = AutomaticLoggerHostBinding {};

#[allow(dead_code)]
#[doc(hidden)]
pub fn ensure_logger() {
    match log::set_logger(&LOGGER) {
        Ok(_) => {}
        Err(_) => {}
    }
    log::set_max_level(log::LevelFilter::Trace);
}

/// A host binding for the wascc:logging capability
pub struct AutomaticLoggerHostBinding {}

impl Default for AutomaticLoggerHostBinding {
    fn default() -> Self {
        AutomaticLoggerHostBinding {}
    }
}

fn set_binding(binding: &str) {
    *CURRENT_BINDING.write().unwrap() = binding.to_string();
}

/// Sets the current binding for the logger. Note that because the logger
/// is statically available in order to support Rust's `log` macros,
/// the _msot recent binding_ you set is the one that will
/// be active for your logging macros. You can't maintain two different logger
/// bindings that send to two different host bindings, you must instead toggle
/// between them.
pub fn host(binding: &str) -> AutomaticLoggerHostBinding {
    set_binding(binding);
    AutomaticLoggerHostBinding {}
}

/// Resets the current logger binding name to the default.
pub fn default() -> AutomaticLoggerHostBinding {
    set_binding("default");
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
            &CURRENT_BINDING.read().unwrap(),
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
            &CURRENT_BINDING.read().unwrap(),
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
            &CURRENT_BINDING.read().unwrap(),
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
            &CURRENT_BINDING.read().unwrap(),
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
            &CURRENT_BINDING.read().unwrap(),
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
            &CURRENT_BINDING.read().unwrap(),
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
            &CURRENT_BINDING.read().unwrap(),
            CAPID_LOGGING,
            OP_LOG,
            &serialize(l)?,
        );
        Ok(())
    }
}
