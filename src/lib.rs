#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
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

//! # wascc-actor
//!
//! The `wascc-actor` library provides WebAssembly module developers with access to the
//! wascc host runtime. Each actor module has a single receive function, declared with the
//!  `actor_receive!` macro. Inside this receive function, the actor
//! module should check the operation of the delivered message and handle it accordingly,
//! returning any binary payload in response. It is the responsibility of the actor module to ensure
//! that the capability provider will be able to understand whichever messages it sends.
//!
//! # Example
//! ```
//! extern crate wascc_actor as actor;
//!
//! use actor::prelude::*;
//!
//! actor_receive!(receive);
//!
//! pub fn receive(ctx: &CapabilitiesContext, operation: &str, msg: &[u8]) -> ReceiveResult {
//!     match operation {
//!         http::OP_HANDLE_REQUEST => hello_world(ctx, msg),
//!         core::OP_HEALTH_REQUEST => Ok(vec![]),
//!         _ => Err("bad dispatch".into()),
//!     }     
//! }
//!
//! fn hello_world(
//!    _ctx: &CapabilitiesContext,
//!    _msg: &[u8]) -> ReceiveResult {
//!     Ok(vec![])
//! }
//! ```

pub type Result<T> = ::std::result::Result<T, crate::errors::Error>;
pub type ReceiveResult = ::std::result::Result<Vec<u8>, Box<dyn std::error::Error>>;

pub extern crate prost;
pub extern crate wapc_guest as wapc;
use crate::kv::DefaultKeyValueStore;
use crate::msg::DefaultMessageBroker;
use crate::objectstore::DefaultObjectStore;
use crate::raw::DefaultRawCapability;
use wapc_guest::console_log;
use wascc_codec::blobstore::{Blob, BlobList, Container, Transfer};

/// Utility function to easily convert a prost Message into a byte vector
pub fn protobytes(msg: impl prost::Message) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(buf)
}

/// Actor developers will use this macro to set up their central receive function
#[macro_export]
macro_rules! actor_receive {
    ($user_handler:ident) => {
        use $crate::wapc::prelude::*;

        wapc_handler!(handle_wapc);
        fn handle_wapc(operation: &str, msg: &[u8]) -> CallResult {
            let ctx = $crate::CapabilitiesContext::new();
            $user_handler(&ctx, &operation, msg).map_err(|e| e.into())
        }
    };
}

/// Represents an abstraction around a client consuming a Key-Value store provided by the host
pub trait KeyValueStore {
    /// Retrieves the value for a given key, returning the value and whether or not it existed.
    fn get(&self, key: &str) -> Result<Option<String>>;
    /// Sets the value for a given key
    fn set(&self, key: &str, value: &str, expires: Option<u32>) -> Result<()>;
    /// Performs an atomic add operation, returning the new value
    fn atomic_add(&self, key: &str, value: i32) -> Result<i32>;
    /// Adds a string value to a list stored within a given key
    fn list_add(&self, key: &str, item: &str) -> Result<usize>;
    /// Deletes all occurrences of an item in a list
    fn list_del_item(&self, key: &str, item: &str) -> Result<usize>;
    /// Deletes the given key
    fn del_key(&self, key: &str) -> Result<()>;
    /// Requests a list of values contained within a given key
    fn list_range(&self, key: &str, start: isize, stop_inclusive: isize) -> Result<Vec<String>>;
    /// Clears a list
    fn list_clear(&self, key: &str) -> Result<()>;
    /// Adds an item to a set
    fn set_add(&self, key: &str, value: &str) -> Result<usize>;
    /// Removes an item from a set
    fn set_remove(&self, key: &str, value: &str) -> Result<usize>;
    /// Returns the union of sets indicated by list of keys
    fn set_union(&self, keys: Vec<String>) -> Result<Vec<String>>;
    /// Returns the intersection of all sets indicated by the list of keys
    fn set_intersect(&self, keys: Vec<String>) -> Result<Vec<String>>;
    /// Returns all members of a given set
    fn set_members(&self, key: &str) -> Result<Vec<String>>;
    /// Indicates whether or not a given key exists in the data store
    fn exists(&self, key: &str) -> Result<bool>;
}

/// Represents an abstraction around a client consuming a message broker provided by the host
pub trait MessageBroker {
    /// Publishes a new message on the given subject with an optional reply-to
    fn publish(&self, subject: &str, reply_to: Option<&str>, payload: &[u8]) -> Result<()>;

    /// Publishes a message on the given subject and awaits a reply on an inbox subject
    fn request(&self, subject: &str, payload: &[u8], timeout_ms: u64) -> Result<Vec<u8>>;
}

pub trait ObjectStore {
    /// Creates a new container
    fn create_container(&self, name: &str) -> Result<Container>;

    /// Removes a container
    fn remove_container(&self, name: &str) -> Result<()>;

    /// Removes an object from a container
    fn remove_object(&self, id: &str, container: &str) -> Result<()>;

    /// Lists objects in a container
    fn list_objects(&self, container: &str) -> Result<BlobList>;

    /// Gets information for a single object
    fn get_blob_info(&self, container: &str, id: &str) -> Result<Option<Blob>>;

    /// Starts an upload to the object store
    fn start_upload(&self, blob: &Blob, chunk_size: u64, total_bytes: u64) -> Result<Transfer>;

    /// Uploads one chunk of a blob (max size determined by blob store capability provider)
    fn upload_chunk(&self, transfer: &Transfer, offset: u64, bytes: &[u8]) -> Result<()>;

    /// Requests a download of a blob, actor will begin receiving OP_RECEIVE_CHUNK messages
    fn start_download(&self, blob: &Blob, chunk_size: u64) -> Result<Transfer>;
}

/// A loosely typed, opaque client consuming a capability provider in the host runtime
pub trait RawCapability {
    fn call(&self, capid: &str, operation: &str, msg: &[u8]) -> Result<Vec<u8>>;
}

/// The capabilities context is the gateway through which all actors communicate with a host runtime. A reference
/// to a capabilities context is passed to the receive function defined by the actor. Individual capabilities are separated
/// through function calls for each capability provider, including any bound opaque `raw` providers.
pub struct CapabilitiesContext {
    kv: Box<dyn KeyValueStore>,
    msg: Box<dyn MessageBroker>,
    raw: Box<dyn RawCapability>,
    blob: Box<dyn ObjectStore>,
}

impl Default for CapabilitiesContext {
    fn default() -> CapabilitiesContext {
        CapabilitiesContext {
            kv: Box::new(DefaultKeyValueStore::new()),
            msg: Box::new(DefaultMessageBroker::new()),
            raw: Box::new(DefaultRawCapability::new()),
            blob: Box::new(DefaultObjectStore::new()),
        }
    }
}

impl CapabilitiesContext {
    /// Creates a new capabilities context. This is invoked by the `actor_receive` macro
    pub fn new() -> CapabilitiesContext {
        CapabilitiesContext {
            kv: Box::new(DefaultKeyValueStore::new()),
            msg: Box::new(DefaultMessageBroker::new()),
            raw: Box::new(DefaultRawCapability::new()),
            blob: Box::new(DefaultObjectStore::new()),
        }
    }

    /// Creates a custom capabilities context. This should be invoked by unit tests looking
    /// to test a receive function with mock capabilities
    pub fn custom(
        kv: impl KeyValueStore + 'static,
        msg: impl MessageBroker + 'static,
        raw: impl RawCapability + 'static,
        blob: impl ObjectStore + 'static,
    ) -> Self {
        CapabilitiesContext {
            kv: Box::new(kv),
            msg: Box::new(msg),
            raw: Box::new(raw),
            blob: Box::new(blob),
        }
    }

    pub fn kv(&self) -> &dyn KeyValueStore {
        self.kv.as_ref()
    }

    pub fn msg(&self) -> &dyn MessageBroker {
        self.msg.as_ref()
    }

    pub fn raw(&self) -> &dyn RawCapability {
        self.raw.as_ref()
    }

    pub fn objectstore(&self) -> &dyn ObjectStore {
        self.blob.as_ref()
    }

    pub fn log(&self, msg: &str) {
        console_log(msg);
    }
}

pub(crate) fn route(capid: &str, op: &str) -> String {
    format!("{}!{}", capid, op)
}

pub mod errors;
pub mod kv;
pub mod msg;
pub mod objectstore;
pub mod prelude;
pub mod raw;
