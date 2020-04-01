use crate::Result;
use std::collections::HashMap;
use wapc_guest::host_call;
use wascc_codec::eventstreams::*;
use wascc_codec::{deserialize, serialize};

const CAPID_EVENTS: &str = "wascc:eventstreams";

pub fn host(binding: &str) -> EventStreamsHostBinding {
    EventStreamsHostBinding {
        binding: binding.to_string(),
    }
}

pub fn default() -> EventStreamsHostBinding {
    EventStreamsHostBinding {
        binding: "default".to_string(),
    }
}

pub struct EventStreamsHostBinding {
    binding: String,
}

impl EventStreamsHostBinding {
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

    pub fn read_all(&self, stream: &str) -> Result<Vec<Event>> {
        let query = StreamQuery {
            count: 0,
            range: None,
            stream_id: stream.to_string(),
        };
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
}
