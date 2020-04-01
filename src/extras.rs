use wapc_guest::host_call;
use wascc_codec::extras::*;
use wascc_codec::{deserialize, serialize};

const CAPID_EXTRAS: &str = "wascc:extras";

pub struct ExtrasHostBinding {
    binding: String,
}

pub fn host(binding: &str) -> ExtrasHostBinding {
    ExtrasHostBinding {
        binding: binding.to_string(),
    }
}

pub fn default() -> ExtrasHostBinding {
    ExtrasHostBinding {
        binding: "default".to_string(),
    }
}

impl ExtrasHostBinding {
    pub fn get_random(&self, min: u32, max: u32) -> crate::Result<u32> {
        let cmd = GeneratorRequest {
            typ: GeneratorRequestType::RandomNumber(min, max),
        };

        host_call(
            &self.binding,
            CAPID_EXTRAS,
            OP_REQUEST_RANDOM,
            &serialize(cmd)?,
        )
        .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
        .map(|r| match r.value {
            GeneratorResultType::RandomNumber(n) => n,
            _ => 0,
        })
        .map_err(|e| e.into())
    }

    pub fn get_guid(&self) -> crate::Result<String> {
        let cmd = GeneratorRequest {
            typ: GeneratorRequestType::Guid,
        };
        host_call(
            &self.binding,
            CAPID_EXTRAS,
            OP_REQUEST_GUID,
            &serialize(cmd)?,
        )
        .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
        .map(|r| match r.value {
            GeneratorResultType::Guid(g) => g,
            _ => "BADGUID".to_string(),
        })
        .map_err(|e| e.into())
    }

    pub fn get_sequence_number(&self) -> crate::Result<u64> {
        let cmd = GeneratorRequest {
            typ: GeneratorRequestType::SequenceNumber,
        };
        host_call(
            &self.binding,
            CAPID_EXTRAS,
            OP_REQUEST_SEQUENCE,
            &serialize(cmd)?,
        )
        .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
        .map(|r| match r.value {
            GeneratorResultType::SequenceNumber(n) => n,
            _ => 0,
        })
        .map_err(|e| e.into())
    }
}
