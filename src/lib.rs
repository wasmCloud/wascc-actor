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
//! actor_handlers!{ codec::http::OP_HANDLE_REQUEST => hello_world,
//!                  codec::core::OP_HEALTH_REQUEST => health }
//!
//! pub fn hello_world(_req: codec::http::Request) -> ReceiveResult {
//!   Ok(vec![])
//! }
//!
//! pub fn health(_req: codec::core::HealthRequest) -> ReceiveResult {
//!   Ok(vec![])
//! }
//! ```

#[macro_use]
extern crate lazy_static;

pub type Result<T> = ::std::result::Result<T, crate::errors::Error>;
pub type ReceiveResult = ::std::result::Result<Vec<u8>, Box<dyn std::error::Error>>;
extern crate log;
pub extern crate wapc_guest as wapc;

use wapc_guest::console_log;

/// Actor developers will use this macro to set up their operation handlers
#[macro_export]
macro_rules! actor_handlers(
    { $($key:path => $user_handler:ident),* } => {
        use $crate::wapc::prelude::*;
        let _ = $crate::logger::ENSURE_LOGGER;
        wapc_handler!(handle_wapc);
        fn handle_wapc(operation: &str, msg: &[u8]) -> CallResult {
            match operation {
                $( $key => $user_handler(deserialize(msg)?).map_err(|e| e.into()), )*
                _ => Err("bad dispatch".into())
            }
        }

     };
);

/// Use this function for simple, unstructured logging outside the usual log macros
pub fn println(msg: &str) {
    console_log(msg)
}

pub mod errors;
pub mod events;
pub mod extras;
pub mod keyvalue;
pub mod logger;
pub mod messaging;
pub mod objectstore;
pub mod prelude;
pub mod untyped;
