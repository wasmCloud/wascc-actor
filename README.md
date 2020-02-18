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

actor_receive!(receive);

pub fn receive(ctx: &CapabilitiesContext, operation: &str, msg: &[u8]) -> ReceiveResult {
    match operation {
        http::OP_HANDLE_REQUEST => hello_world(ctx, msg),
        core::OP_HEALTH_REQUEST => Ok(vec![]),
        _ => Err("bad dispatch".into()),
    }     
}

fn hello_world(
   _ctx: &CapabilitiesContext,
   _msg: &[u8]) -> ReceiveResult {

    // Utilize capabilities via the context here
    Ok(vec![])
}
```
