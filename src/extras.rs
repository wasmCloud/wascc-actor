use crate::Extras;
use wapc_guest::host_call;
use wascc_codec::extras::*;
use wascc_codec::{deserialize, serialize};

/// The reserved capability ID for the extras functionality
pub const CAPID_EXTRAS: &str = "wascc:extras";

pub struct DefaultExtras {}

impl DefaultExtras {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DefaultExtras {
    fn default() -> Self {
        DefaultExtras {}
    }
}

impl Extras for DefaultExtras {
    fn get_random(&self, min: u32, max: u32) -> crate::Result<u32> {
        let cmd = GeneratorRequest {
            typ: GeneratorRequestType::RandomNumber(min, max),
        };

        host_call(CAPID_EXTRAS, OP_REQUEST_RANDOM, &serialize(cmd)?)
            .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
            .map(|r| match r.value {
                GeneratorResultType::RandomNumber(n) => n,
                _ => 0,
            })
            .map_err(|e| e.into())
    }

    fn get_guid(&self) -> crate::Result<String> {
        let cmd = GeneratorRequest {
            typ: GeneratorRequestType::Guid,
        };
        host_call(CAPID_EXTRAS, OP_REQUEST_GUID, &serialize(cmd)?)
            .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
            .map(|r| match r.value {
                GeneratorResultType::Guid(g) => g,
                _ => "BADGUID".to_string(),
            })
            .map_err(|e| e.into())
    }

    fn get_sequence_number(&self) -> crate::Result<u64> {
        let cmd = GeneratorRequest {
            typ: GeneratorRequestType::SequenceNumber,
        };
        host_call(CAPID_EXTRAS, OP_REQUEST_SEQUENCE, &serialize(cmd)?)
            .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
            .map(|r| match r.value {
                GeneratorResultType::SequenceNumber(n) => n,
                _ => 0,
            })
            .map_err(|e| e.into())
    }
}
