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
//! This module contains types and utility functions for eventstream handling.

use crate::Result;
use codec::eventstreams::*;
use codec::{deserialize, serialize};
use std::collections::HashMap;
use wapc_guest::host_call;
use wascc_codec as codec;

const CAPID_EVENTS: &str = "wascc:eventstreams";

/// Creates a new host binding for an event stream capability provider
pub fn host(binding: &str) -> EventStreamsHostBinding {
    EventStreamsHostBinding {
        binding: binding.to_string(),
    }
}

/// Creates a default host binding for an event stream capability provider
pub fn default() -> EventStreamsHostBinding {
    EventStreamsHostBinding {
        binding: "default".to_string(),
    }
}

/// A host binding for an event stream capability provider
pub struct EventStreamsHostBinding {
    binding: String,
}

impl EventStreamsHostBinding {
    /// Writes the given event (a collection of key-value pairs) to a named stream
    pub fn write_event(&self, stream: &str, values: HashMap<String, String>) -> Result<String> {
        let ev = Event {
            event_id: "".to_string(),
            stream: stream.to_string(),
            values,
        };

        host_call(&self.binding, CAPID_EVENTS, OP_WRITE_EVENT, &serialize(ev)?)
            .map(|v| {
                deserialize::<WriteResponse>(&v)
                    .unwrap()
                    .event_id
                    .to_string()
            })
            .map_err(|e| e.into())
    }

    /// Reads all available events from the given stream
    pub fn read_all(&self, stream: &str) -> Result<Vec<Event>> {
        let query = self.generate_query(0, stream, None);
        self.execute_query(query)
    }

    /// Reads all available events from a given stream up to a given maximum number.
    /// May return less than the specified limit if less than that exist on the stream
    pub fn read_limit(&self, stream: &str, limit: u64) -> Result<Vec<Event>> {
        let query = self.generate_query(limit, stream, None);
        self.execute_query(query)
    }

    fn execute_query(&self, query: StreamQuery) -> Result<Vec<Event>> {
        host_call(
            &self.binding,
            CAPID_EVENTS,
            OP_QUERY_STREAM,
            &serialize(query)?,
        )
        .map(|v| {
            deserialize::<StreamResults>(v.as_ref())
                .unwrap()
                .events
                .clone()
        })
        .map_err(|e| e.into())
    }

    fn generate_query(&self, count: u64, stream: &str, range: Option<TimeRange>) -> StreamQuery {
        StreamQuery {
            count,
            stream_id: stream.to_string(),
            range,
        }
    }
}
