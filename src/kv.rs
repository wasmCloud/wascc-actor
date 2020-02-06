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

//! # Key-Value Store
//!
//! This module contains the key-value store through which guest modules access
//! the currently bound `wascap:keyvalue` capability provider

use crate::protobytes;
use crate::route;
use crate::KeyValueStore;
use crate::Result;
use codec::keyvalue::*;
use prost::Message;
use wapc_guest::host_call;
use wascc_codec as codec;

/// The reserved capability ID for a key/value store. Used for call routing in the host runtime.
pub const CAPID_KEYVALUE: &str = "wascc:keyvalue";

/// An abstraction around a host runtime capability for a key-value store
pub struct DefaultKeyValueStore {}

impl DefaultKeyValueStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DefaultKeyValueStore {
    fn default() -> Self {
        DefaultKeyValueStore {}
    }
}

impl KeyValueStore for DefaultKeyValueStore {
    fn get(&self, key: &str) -> Result<Option<String>> {
        let cmd = GetRequest {
            key: key.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_GET), &protobytes(cmd)?).map(|vec| {
            let resp = GetResponse::decode(vec.as_ref()).unwrap();
            if resp.exists {
                Some(resp.value)
            } else {
                None
            }
        }).map_err(|e| e.into())
    }

    fn set(&self, key: &str, value: &str, expires: Option<u32>) -> Result<()> {
        let cmd = SetRequest {
            key: key.to_string(),
            value: value.to_string(),
            expires_s: expires.unwrap_or(0) as _,
        };
        host_call(&route(CAPID_KEYVALUE, OP_SET), &protobytes(cmd)?).map(|_vec| ()).map_err(|e| e.into())
    }

    fn atomic_add(&self, key: &str, value: i32) -> Result<i32> {
        let cmd = AddRequest {
            key: key.to_string(),
            value,
        };
        host_call(&route(CAPID_KEYVALUE, OP_ADD), &protobytes(cmd)?).map(|vec| {
            let resp = AddResponse::decode(vec.as_ref()).unwrap();
            resp.value
        }).map_err(|e| e.into())
    }

    fn list_add(&self, key: &str, item: &str) -> Result<usize> {
        let cmd = ListPushRequest {
            key: key.to_string(),
            value: item.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_PUSH), &protobytes(cmd)?).map(|vec| {
            let resp = ListResponse::decode(vec.as_ref()).unwrap();
            resp.new_count as usize
        }).map_err(|e| e.into())
    }

    fn list_del_item(&self, key: &str, item: &str) -> Result<usize> {
        let cmd = ListDelItemRequest {
            key: key.to_string(),
            value: item.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_LIST_DEL), &protobytes(cmd)?).map(|vec| {
            let resp = ListResponse::decode(vec.as_ref()).unwrap();
            resp.new_count as usize
        }).map_err(|e| e.into())
    }

    fn del_key(&self, key: &str) -> Result<()> {
        let cmd = DelRequest {
            key: key.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_DEL), &protobytes(cmd)?).map(|_vec| ()).map_err(|e| e.into())
    }

    fn list_range(&self, key: &str, start: isize, stop_inclusive: isize) -> Result<Vec<String>> {
        let cmd = ListRangeRequest {
            key: key.to_string(),
            start: start as i32,
            stop: stop_inclusive as i32,
        };
        host_call(&route(CAPID_KEYVALUE, OP_RANGE), &protobytes(cmd)?).map(|vec| {
            let resp = ListRangeResponse::decode(vec.as_ref()).unwrap();
            resp.values
        }).map_err(|e| e.into())
    }

    fn list_clear(&self, key: &str) -> Result<()> {
        let cmd = ListClearRequest {
            key: key.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_CLEAR), &protobytes(cmd)?).map(|_vec| ()).map_err(|e| e.into())
    }

    fn set_add(&self, key: &str, value: &str) -> Result<usize> {
        let cmd = SetAddRequest {
            key: key.to_string(),
            value: value.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_SET_ADD), &protobytes(cmd)?).map(|vec| {
            let resp = SetOperationResponse::decode(vec.as_ref()).unwrap();
            resp.new_count as usize
        }).map_err(|e| e.into())
    }

    fn set_remove(&self, key: &str, value: &str) -> Result<usize> {
        let cmd = SetRemoveRequest {
            key: key.to_string(),
            value: value.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_SET_REMOVE), &protobytes(cmd)?).map(|vec| {
            let resp = SetOperationResponse::decode(vec.as_ref()).unwrap();
            resp.new_count as usize
        }).map_err(|e| e.into())
    }

    fn set_union(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let cmd = SetUnionRequest { keys };
        host_call(&route(CAPID_KEYVALUE, OP_SET_UNION), &protobytes(cmd)?).map(|vec| {
            let resp = SetQueryResponse::decode(vec.as_ref()).unwrap();
            resp.values
        }).map_err(|e| e.into())
    }

    fn set_intersect(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let cmd = SetIntersectionRequest { keys };
        host_call(&route(CAPID_KEYVALUE, OP_SET_INTERSECT), &protobytes(cmd)?).map(|vec| {
            let resp = SetQueryResponse::decode(vec.as_ref()).unwrap();
            resp.values
        }).map_err(|e| e.into())
    }

    fn set_members(&self, key: &str) -> Result<Vec<String>> {
        let cmd = SetQueryRequest {
            key: key.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_SET_QUERY), &protobytes(cmd)?).map(|vec| {
            let resp = SetQueryResponse::decode(vec.as_ref()).unwrap();
            resp.values
        }).map_err(|e| e.into())
    }

    fn exists(&self, key: &str) -> Result<bool> {
        let cmd = KeyExistsQuery {
            key: key.to_string(),
        };
        host_call(&route(CAPID_KEYVALUE, OP_KEY_EXISTS), &protobytes(cmd)?).map(|vec| {
            let resp = GetResponse::decode(vec.as_ref()).unwrap();
            resp.exists
        }).map_err(|e| e.into())
    }
}
