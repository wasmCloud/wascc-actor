[![crates.io](https://img.shields.io/crates/v/wascc-actor.svg)](https://crates.io/crates/wascc-actor)&nbsp;
![Rust](https://github.com/wascc/wascc-actor/workflows/Rust/badge.svg)&nbsp;
![license](https://img.shields.io/crates/l/wascc-actor.svg)&nbsp;
[![documentation](https://docs.rs/wascc-actor/badge.svg)](https://docs.rs/wascc-actor)

# DEPRECATED
This crate is no longer used as of 0.15.0. For actors targeting the 0.15.0 wasmCloud host, you will want to use waPC-generated types and wrappers that can be found in the [actor-interfaces](https://github.com/wascc/actor-interfaces) repository.

# WebAssembly Secure Capabilities Connector - Actor SDK

The [waSCC Actor SDK](https://wascc.dev) is used by Rust developers building cloud-native workloads for the `wasm32-unknown-unknown` target. Using waSCC to host your WebAssembly module frees you from the burden of manually implementing traditional non-functional requirements and boilerplate that typically bogs down development time. waSCC lets you focus solely on writing the business logic in a portable, secure wasm module that can run anywhere there's a waSCC host.

For more documentation, tutorials, and examples, please check out the [wascc](https://wascc.dev) website.

# Example

```rust
extern crate wascc_actor as actor;

use actor::prelude::*;

actor_handlers!{ codec::http::OP_HANDLE_REQUEST => hello_world, 
                 codec::core::OP_HEALTH_REQUEST => health }

fn hello_world(_req: codec::http::Request) -> ReceiveResult {

    // Utilize capabilities here
    // ...
    
    Ok(vec![])
}

fn health(_req: codec::core::HealthRequest) -> ReceiveResult { 
    Ok(vec![])
}
```

## Debug output vs. using the wascc:logging capability

If you want more functionality beyond the simple `println` call, then you can
sign your modules with the `wascc:logging` capability and you'll be able to use the idiomatic Rust `log` macros like `debug!`, `warn!`, `trace!`, etc.
