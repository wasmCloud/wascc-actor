[![crates.io](https://img.shields.io/crates/v/wascc-actor.svg)](https://crates.io/crates/wascc-actor)&nbsp;
![travis](https://travis-ci.org/wascc/wascc-actor.svg?branch=master)&nbsp;
![license](https://img.shields.io/crates/l/wascc-actor.svg)&nbsp;
[![documentation](https://docs.rs/wascc-actor/badge.svg)](https://docs.rs/wascc-actor)

# WebAssembly Secure Capabilities Connector - Actor SDK

The [waSCC Actor SDK](https://wascc.dev) is used by Rust developers building cloud-native workloads for the `wasm32-unknown-unknown` target. Using waSCC to host your WebAssembly module frees you from the burden of manually implementing traditional non-functional requirements and boilerplate that typically bogs down development time. waSCC lets you focus squarely on compiling the business logic in a portable, secure wasm module that can run anywhere there's a waSCC host.

For more documentation, tutorials, and examples, please check out the [wascc](https://wascc.dev) website.

# Example

```rust
extern crate wascc_actor as actor;

use actor::prelude::*;

actor_handlers!{ http::OP_HANDLE_REQUEST => hello_world, 
                 core::OP_HEALTH_REQUEST => health }

fn hello_world(
   _ctx: &CapabilitiesContext,
   _req: http::Request) -> ReceiveResult {

    // Utilize capabilities via the context here
    Ok(vec![])
}

fn health(_ctx: &CapabilitiesContext, _req: core::HealthRequest) -> ReceiveResult { 
    Ok(vec![])
}
```

## Using the wascc:logging capability


Inside a _wascc_ actor, use normal `log` macros to write logs:

```
#[macro_use]
extern crate log;

fn hello_world(
   ctx: &CapabilitiesContext,
   r: http::Request) -> CallResult {
    // regular log macros get intercepted automatically
    // and translated into waPC calls to the wascc:logging 
    // provider
    debug!("Request path {}", r.path);
    
    let echo = EchoRequest {
        method: r.method,
        path: r.path,
        query_string: r.query_string,
        body: r.body,
    };
   
    let resp = http::Response::json(echo, 200, "OK");

    Ok(serialize(resp)?)
}
```
