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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "allow_missing_functions" with type "test" omitted
# Unsupported target "call" with type "example" omitted
# Unsupported target "error_propagation" with type "test" omitted
# Unsupported target "many_instances" with type "bench" omitted
# Unsupported target "nginx" with type "bench" omitted

rust_library(
    name = "wasmer_runtime",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//cargo/vendor/memmap-0.7.0:memmap",
        "//cargo/vendor/serde-1.0.104:serde",
        "//cargo/vendor/serde_derive-1.0.104:serde_derive",
        "//cargo/vendor/wasmer-clif-backend-0.13.1:wasmer_clif_backend",
        "//cargo/vendor/wasmer-runtime-core-0.13.1:wasmer_runtime_core",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.13.1",
    crate_features = [
        "cranelift",
        "default",
        "default-backend-cranelift",
        "wasmer-clif-backend",
    ],
)

