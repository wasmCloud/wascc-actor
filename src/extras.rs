use wapc_guest::host_call;
use wascc_codec::extras::*;
use wascc_codec::{deserialize, serialize};

const CAPID_EXTRAS: &str = "wascc:extras";

/// A hsot binding for the wascc:extras capability
pub struct ExtrasHostBinding {
    binding: String,
}

/// Creates a named host binding for the wascc:extras capability. You should never
/// use this unless you know that a custom host runtime has provided an alternate
/// extras provider.
pub fn host(binding: &str) -> ExtrasHostBinding {
    ExtrasHostBinding {
        binding: binding.to_string(),
    }
}

/// Creates a host binding for the wascc:extras capability
pub fn default() -> ExtrasHostBinding {
    ExtrasHostBinding {
        binding: "default".to_string(),
    }
}

impl ExtrasHostBinding {
    /// Queries the host for a random number within a specified range
    pub fn get_random(&self, min: u32, max: u32) -> crate::HandlerResult<u32> {
        let cmd = GeneratorRequest {
            min,
            max,
            random: true,
            sequence: false,
            guid: false,
        };

        host_call(
            &self.binding,
            CAPID_EXTRAS,
            OP_REQUEST_RANDOM,
            &serialize(cmd)?,
        )
        .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
        .map(|r| r.random_number)
        .map_err(|e| e.into())
    }

    /// Requests a newly generated GUID string from the host
    pub fn get_guid(&self) -> crate::HandlerResult<String> {
        let cmd = GeneratorRequest {
            guid: true,
            random: false,
            sequence: false,
            min: 0,
            max: 0,
        };
        host_call(
            &self.binding,
            CAPID_EXTRAS,
            OP_REQUEST_GUID,
            &serialize(cmd)?,
        )
        .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
        .map(|r| r.guid.unwrap_or("none".to_string()))
        .map_err(|e| e.into())
    }

    /// Requests a sequence number from the host. Note that the sequence number will only be
    /// unique within the host, and is not globally unique
    pub fn get_sequence_number(&self) -> crate::HandlerResult<u64> {
        let cmd = GeneratorRequest {
            sequence: true,
            guid: false,
            random: false,
            min: 0,
            max: 0,
        };
        host_call(
            &self.binding,
            CAPID_EXTRAS,
            OP_REQUEST_SEQUENCE,
            &serialize(cmd)?,
        )
        .map(|v| deserialize::<GeneratorResult>(v.as_ref()).unwrap())
        .map(|r| r.sequence_number)
        .map_err(|e| e.into())
    }
}
