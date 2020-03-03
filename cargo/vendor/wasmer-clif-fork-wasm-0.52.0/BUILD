"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "restricted", # "Apache-2.0 WITH LLVM-exception"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "wasm_testsuite" with type "test" omitted

rust_library(
    name = "wasmer_clif_fork_wasm",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/cranelift-codegen-0.52.0:cranelift_codegen",
        "//cargo/vendor/cranelift-entity-0.52.0:cranelift_entity",
        "//cargo/vendor/log-0.4.8:log",
        "//cargo/vendor/thiserror-1.0.11:thiserror",
        "//cargo/vendor/wasmer-clif-fork-frontend-0.52.0:wasmer_clif_fork_frontend",
        "//cargo/vendor/wasmparser-0.45.2:wasmparser",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.52.0",
    crate_features = [
        "basic-blocks",
        "default",
        "std",
    ],
)

