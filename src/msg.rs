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

//! # Message Broker
//!
//! This module contains the message broker client interface through which actor modules access
//! the currently bound `wascc:messaging` capability provider

use crate::protobytes;
use crate::route;
use crate::MessageBroker;
use wapc_guest::host_call;

/// The reserved capability ID for a message broker. Used for call routing in the host runtime.
pub const CAPID_MESSAGING: &str = "wascc:messaging";

use crate::Result;
use codec::messaging::{
    BrokerMessage, PublishMessage, RequestMessage, OP_PERFORM_REQUEST, OP_PUBLISH_MESSAGE,
};
use wascc_codec as codec;

/// Exposes message broker functionality to actor modules
pub struct DefaultMessageBroker {}

impl DefaultMessageBroker {
    pub fn new() -> Self {
        DefaultMessageBroker::default()
    }
}

impl Default for DefaultMessageBroker {
    fn default() -> Self {
        DefaultMessageBroker {}
    }
}

impl MessageBroker for DefaultMessageBroker {
    fn publish(&self, subject: &str, reply_to: Option<&str>, payload: &[u8]) -> Result<()> {
        let cmd = PublishMessage {
            message: Some(BrokerMessage {
                subject: subject.to_string(),
                reply_to: reply_to.map_or("".to_string(), |r| r.to_string()),
                body: payload.to_vec(),
            }),
        };

        host_call(
            &route(CAPID_MESSAGING, OP_PUBLISH_MESSAGE),
            &protobytes(cmd)?,
        )
        .map(|_vec| ())
    }

    fn request(&self, subject: &str, payload: &[u8], timeout_ms: u64) -> Result<Vec<u8>> {
        let cmd = RequestMessage {
            subject: subject.to_string(),
            timeout_ms: timeout_ms as _,
            body: payload.to_vec(),
        };

        // The broker plugin applies no wrapper around the response from the broker, the
        // raw payload is delivered.
        host_call(
            &route(CAPID_MESSAGING, OP_PERFORM_REQUEST),
            &protobytes(cmd)?,
        )
    }
}
