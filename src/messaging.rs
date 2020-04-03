// Copyright 2015-2020 Capital One Services, LLC
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

//! # Message Broker
//!
//! This module contains the message broker client interface through which actor modules access
//! a bound `wascc:messaging` capability provider

use wapc_guest::host_call;

const CAPID_MESSAGING: &str = "wascc:messaging";

use crate::Result;
use codec::messaging::{BrokerMessage, RequestMessage, OP_PERFORM_REQUEST, OP_PUBLISH_MESSAGE};
use codec::serialize;
use wascc_codec as codec;

/// Create a new named message broker host binding
pub fn host(binding: &str) -> MessageBrokerHostBinding {
    MessageBrokerHostBinding {
        binding: binding.to_string(),
    }
}

/// Create a default message broker host binding
pub fn default() -> MessageBrokerHostBinding {
    MessageBrokerHostBinding {
        binding: "default".to_string(),
    }
}

/// Exposes message broker functionality to actor modules
pub struct MessageBrokerHostBinding {
    binding: String,
}

impl MessageBrokerHostBinding {
    /// Publishes a message on a given subject with an optional reply subject
    pub fn publish(&self, subject: &str, reply_to: Option<&str>, payload: &[u8]) -> Result<()> {
        let cmd = BrokerMessage {
            subject: subject.to_string(),
            reply_to: reply_to.map_or("".to_string(), |r| r.to_string()),
            body: payload.to_vec(),
        };

        host_call(
            &self.binding,
            CAPID_MESSAGING,
            OP_PUBLISH_MESSAGE,
            &serialize(cmd)?,
        )
        .map_err(|e| e.into())
        .map(|_vec| ())
    }

    /// Publishes a message and expects a reply to come back within a given timeout (in milliseconds)
    pub fn request(&self, subject: &str, payload: &[u8], timeout_ms: u64) -> Result<Vec<u8>> {
        let cmd = RequestMessage {
            subject: subject.to_string(),
            timeout_ms: timeout_ms as _,
            body: payload.to_vec(),
        };

        // The broker plugin applies no wrapper around the response from the broker, the
        // raw payload is delivered.
        host_call(
            &self.binding,
            CAPID_MESSAGING,
            OP_PERFORM_REQUEST,
            &serialize(cmd)?,
        )
        .map_err(|e| e.into())
    }
}
