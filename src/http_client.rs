//! # HTTP Client
//!
//! This module contains the HTTP client through which actors consume
//! the currently bound `wascap:http_client` capability provider

use crate::Result;
use wapc_guest::host_call;
use wascc_codec::{deserialize, http::*, serialize};

const CAPID_HTTPCLIENT: &str = "wascc:http_client";

/// An abstraction around a host runtime capability for an HTTP client
pub struct HttpClientHostBinding {
    binding: String,
}

impl Default for HttpClientHostBinding {
    fn default() -> Self {
        HttpClientHostBinding {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding for the HTTP client capability
pub fn host(binding: &str) -> HttpClientHostBinding {
    HttpClientHostBinding {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding for the key-value store capability
pub fn default() -> HttpClientHostBinding {
    HttpClientHostBinding::default()
}

impl HttpClientHostBinding {
    pub fn request(&self, request: Request) -> Result<Response> {
        host_call(
            &self.binding,
            CAPID_HTTPCLIENT,
            OP_PERFORM_REQUEST,
            &serialize(request)?,
        )
        .map(|r| deserialize::<Response>(r.as_ref()).unwrap())
        .map_err(|e| e.into())
    }
}
