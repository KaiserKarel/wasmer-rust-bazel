# wasmer-rust-bazel

Experimental repo to evaluate the steps needed to support Bazel builds for the Wasmer-rust
project. 

To fully support Bazel, we must be able to build a wasm function and it's dependencies
using Bazel, thus producing a .wasm, and building the serverside VM, which requires building
the [wasmer-runtime](https://github.com/wasmerio/wasmer/tree/master/lib/runtime) and 
[wasmer-runtime-core](https://github.com/wasmerio/wasmer/tree/master/lib/runtime-core).

In general, Bazel configurations for Rust are generated using [cargo raze](https://github.com/google/cargo-raze) and 
[rules_rust](https://github.com/bazelbuild/rules_rust). This workflow looks as follows:

1. Maintain a global Cargo.toml containing all dependencies.
2. Generate a lockfile and vendor.
3. Let `cargo raze` generate BUILD files for each vendored dependency.
4. Use these generated BUILD files within user's projects.

For standard crates; this workflow requires no user modification. The problem
lies in so called 'snowflake' crates with complicated `build.rs` files, or crates
which heavily rely on Cargo provided environment variables. `cargo raze` and rules_rust
attempt to provide some of Cargo's functionality, but since Bazel builds are reproducible 
(and by defintion crates using `build.rs` are not), not all functionality can be provided.

`cargo raze` will attempt to generate and run `build.rs` files, but they are more
limited.

Inside wasmer-rust-example, a modified of the [official](https://github.com/wasmerio/wasmer-rust-example)
is present, which we attempt to build using Bazel.
