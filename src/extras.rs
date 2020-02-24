use crate::{protobytes, route, Extras};
use prost::Message;
use wapc_guest::host_call;
use wascc_codec::extras::*;

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
            min,
            max,
            gentype: GeneratorType::Random as i32,
        };

        host_call(&route(CAPID_EXTRAS, OP_REQUEST_RANDOM), &protobytes(cmd)?)
            .map(|v| GeneratorResult::decode(v.as_ref()).unwrap())
            .map(|r| match r.value.unwrap() {
                generator_result::Value::RandomNo(n) => n,
                _ => 0,
            })
            .map_err(|e| e.into())
    }

    fn get_guid(&self) -> crate::Result<String> {
        let cmd = GeneratorRequest {
            min: 0,
            max: 0,
            gentype: GeneratorType::Guid as i32,
        };
        host_call(&route(CAPID_EXTRAS, OP_REQUEST_GUID), &protobytes(cmd)?)
            .map(|v| GeneratorResult::decode(v.as_ref()).unwrap())
            .map(|r| match r.value.unwrap() {
                generator_result::Value::Guid(g) => g,
                _ => "BADGUID".to_string(),
            })
            .map_err(|e| e.into())
    }

    fn get_sequence_number(&self) -> crate::Result<u64> {
        let cmd = GeneratorRequest {
            min: 0,
            max: 0,
            gentype: GeneratorType::Sequence as i32,
        };
        host_call(&route(CAPID_EXTRAS, OP_REQUEST_SEQUENCE), &protobytes(cmd)?)
            .map(|v| GeneratorResult::decode(v.as_ref()).unwrap())
            .map(|r| match r.value.unwrap() {
                generator_result::Value::SequenceNo(n) => n,
                _ => 0,
            })
            .map_err(|e| e.into())
    }
}
