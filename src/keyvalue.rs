//! # Key-Value Store
//!
//! This module contains the key-value store through which guest modules access
//! the currently bound `wascap:keyvalue` capability provider

use codec::keyvalue::*;
use codec::{deserialize, serialize};
use wapc_guest::host_call;
use wascc_codec as codec;

use crate::HandlerResult;

const CAPID_KEYVALUE: &str = "wascc:keyvalue";

/// An abstraction around a host runtime capability for a key-value store
pub struct KeyValueStoreHostBinding {
    binding: String,
}

impl Default for KeyValueStoreHostBinding {
    fn default() -> Self {
        KeyValueStoreHostBinding {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding for the key-value store capability
pub fn host(binding: &str) -> KeyValueStoreHostBinding {
    KeyValueStoreHostBinding {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding for the key-value store capability
pub fn default() -> KeyValueStoreHostBinding {
    KeyValueStoreHostBinding::default()
}

impl KeyValueStoreHostBinding {
    /// Obtains a single value from the store
    pub fn get(&self, key: &str) -> HandlerResult<Option<String>> {
        let cmd = GetRequest {
            key: key.to_string(),
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_GET, &serialize(cmd)?)
            .map(|vec| {
                let resp = deserialize::<GetResponse>(vec.as_ref()).unwrap();
                if resp.exists {
                    Some(resp.value)
                } else {
                    None
                }
            })
            .map_err(|e| e.into())
    }

    /// Sets a value in the store
    pub fn set(&self, key: &str, value: &str, expires: Option<u32>) -> HandlerResult<()> {
        let cmd = SetRequest {
            key: key.to_string(),
            value: value.to_string(),
            expires_s: expires.unwrap_or(0) as _,
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_SET, &serialize(cmd)?)
            .map(|_vec| ())
            .map_err(|e| e.into())
    }

    /// Performs an atomic increment operation
    pub fn atomic_add(&self, key: &str, value: i32) -> HandlerResult<i32> {
        let cmd = AddRequest {
            key: key.to_string(),
            value,
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_ADD, &serialize(cmd)?)
            .map(|vec| {
                let resp = deserialize::<AddResponse>(vec.as_ref()).unwrap();
                resp.value
            })
            .map_err(|e| e.into())
    }

    /// Adds an item to a list at the given key
    pub fn list_add(&self, key: &str, item: &str) -> HandlerResult<usize> {
        let cmd = ListPushRequest {
            key: key.to_string(),
            value: item.to_string(),
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_PUSH, &serialize(cmd)?)
            .map(|vec| {
                let resp = deserialize::<ListResponse>(vec.as_ref()).unwrap();
                resp.new_count as usize
            })
            .map_err(|e| e.into())
    }

    /// Removes an item from the list at the given key
    pub fn list_del_item(&self, key: &str, item: &str) -> HandlerResult<usize> {
        let cmd = ListDelItemRequest {
            key: key.to_string(),
            value: item.to_string(),
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_LIST_DEL, &serialize(cmd)?)
            .map(|vec| {
                let resp = deserialize::<ListResponse>(vec.as_ref()).unwrap();
                resp.new_count as usize
            })
            .map_err(|e| e.into())
    }

    /// Removes the data associated with a given key, which can include lists or sets
    pub fn del_key(&self, key: &str) -> HandlerResult<()> {
        let cmd = DelRequest {
            key: key.to_string(),
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_DEL, &serialize(cmd)?)
            .map(|_vec| ())
            .map_err(|e| e.into())
    }

    /// Queries a given list-type key for a range of values
    pub fn list_range(
        &self,
        key: &str,
        start: isize,
        stop_inclusive: isize,
    ) -> HandlerResult<Vec<String>> {
        let cmd = ListRangeRequest {
            key: key.to_string(),
            start: start as i32,
            stop: stop_inclusive as i32,
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_RANGE, &serialize(cmd)?)
            .map(|vec| {
                let resp = deserialize::<ListRangeResponse>(vec.as_ref()).unwrap();
                resp.values
            })
            .map_err(|e| e.into())
    }

    /// Clears a list while leaving the key intact
    pub fn list_clear(&self, key: &str) -> HandlerResult<()> {
        let cmd = ListClearRequest {
            key: key.to_string(),
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_CLEAR, &serialize(cmd)?)
            .map(|_vec| ())
            .map_err(|e| e.into())
    }

    /// Adds a value to a set at the given key
    pub fn set_add(&self, key: &str, value: &str) -> HandlerResult<usize> {
        let cmd = SetAddRequest {
            key: key.to_string(),
            value: value.to_string(),
        };
        host_call(&self.binding, CAPID_KEYVALUE, OP_SET_ADD, &serialize(cmd)?)
            .map(|vec| {
                let resp = deserialize::<SetOperationResponse>(vec.as_ref()).unwrap();
                resp.new_count as usize
            })
            .map_err(|e| e.into())
    }

    /// Removes a value from the given set
    pub fn set_remove(&self, key: &str, value: &str) -> HandlerResult<usize> {
        let cmd = SetRemoveRequest {
            key: key.to_string(),
            value: value.to_string(),
        };
        host_call(
            &self.binding,
            CAPID_KEYVALUE,
            OP_SET_REMOVE,
            &serialize(cmd)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetOperationResponse>(vec.as_ref()).unwrap();
            resp.new_count as usize
        })
        .map_err(|e| e.into())
    }

    /// Performs a union of sets specified by the list of keys
    pub fn set_union(&self, keys: Vec<String>) -> HandlerResult<Vec<String>> {
        let cmd = SetUnionRequest { keys };
        host_call(
            &self.binding,
            CAPID_KEYVALUE,
            OP_SET_UNION,
            &serialize(cmd)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetQueryResponse>(vec.as_ref()).unwrap();
            resp.values
        })
        .map_err(|e| e.into())
    }

    /// Performs the intersection of sets specified by the given keys
    pub fn set_intersect(&self, keys: Vec<String>) -> HandlerResult<Vec<String>> {
        let cmd = SetIntersectionRequest { keys };
        host_call(
            &self.binding,
            CAPID_KEYVALUE,
            OP_SET_INTERSECT,
            &serialize(cmd)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetQueryResponse>(vec.as_ref()).unwrap();
            resp.values
        })
        .map_err(|e| e.into())
    }

    /// Returns a list of members belonging to a given set
    pub fn set_members(&self, key: &str) -> HandlerResult<Vec<String>> {
        let cmd = SetQueryRequest {
            key: key.to_string(),
        };
        host_call(
            &self.binding,
            CAPID_KEYVALUE,
            OP_SET_QUERY,
            &serialize(cmd)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetQueryResponse>(vec.as_ref()).unwrap();
            resp.values
        })
        .map_err(|e| e.into())
    }

    /// Indicates whether a key exists (not that empty lists/sets may return true for their
    /// existence if they were cleared instead of deleted)
    pub fn exists(&self, key: &str) -> HandlerResult<bool> {
        let cmd = KeyExistsQuery {
            key: key.to_string(),
        };
        host_call(
            &self.binding,
            CAPID_KEYVALUE,
            OP_KEY_EXISTS,
            &serialize(cmd)?,
        )
        .map(|vec| {
            let resp = deserialize::<GetResponse>(vec.as_ref()).unwrap();
            resp.exists
        })
        .map_err(|e| e.into())
    }
}
