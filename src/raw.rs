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

//! # Raw capability provider interface
//!
//! This module contains the raw capability provider client. Use this facility
//! if you are building your own internal or proprietary capability provider that is not one of
//! the provider types exposed as part of the standard actor SDK. You might want to provide your own
//! wrapper crate that depends on this one and provide a strongly-typed layer around the raw call for
//! a better developer experience

use crate::RawCapability;
use crate::Result;
use wapc_guest::host_call;

/// An implementation of the opaque or "loosely typed" capability provider which depends upon the host runtime
pub struct DefaultRawCapability {}

impl DefaultRawCapability {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DefaultRawCapability {
    fn default() -> Self {
        DefaultRawCapability {}
    }
}

impl RawCapability for DefaultRawCapability {
    /// Performs a host call carrying an opaque binary payload. Only use this function
    /// if you know the host runtime is carrying a capability provider that can understand
    /// this payload
    fn call(&self, capid: &str, operation: &str, msg: &[u8]) -> Result<Vec<u8>> {
        host_call(capid, operation, msg).map_err(|e| e.into())
    }
}
