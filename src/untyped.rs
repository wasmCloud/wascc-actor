//! # Message Broker
//!
//! This module contains the message broker client interface through which actor modules access
//! a bound `wascc:messaging` capability provider

use wapc_guest::host_call;

use crate::HandlerResult;

/// An untyped (or "raw") host binding. This allows the actor to send arbitary binary
/// payloads with named operations. This will likely be wrapped by a fit-for-purpose
/// API that uses types outside the scope of waSCC's first-party capabilities.
pub struct UntypedHostBinding {
    binding: String,
}

/// Create a new named untyped/raw host binding
pub fn host(binding: &str) -> UntypedHostBinding {
    UntypedHostBinding {
        binding: binding.to_string(),
    }
}

/// Create a default untyped/raw host binding
pub fn default() -> UntypedHostBinding {
    UntypedHostBinding {
        binding: "default".to_string(),
    }
}

impl UntypedHostBinding {
    /// Invoke the given operation on the target capability ID with the specified payload
    pub fn call(&self, capid: &str, operation: &str, payload: Vec<u8>) -> HandlerResult<Vec<u8>> {
        host_call(&self.binding, capid, operation, &payload).map_err(|e| e.into())
    }
}
