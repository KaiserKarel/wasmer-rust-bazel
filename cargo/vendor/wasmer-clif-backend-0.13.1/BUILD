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



rust_library(
    name = "wasmer_clif_backend",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/byteorder-1.3.4:byteorder",
        "//cargo/vendor/cranelift-codegen-0.52.0:cranelift_codegen",
        "//cargo/vendor/cranelift-entity-0.52.0:cranelift_entity",
        "//cargo/vendor/cranelift-native-0.52.0:cranelift_native",
        "//cargo/vendor/libc-0.2.67:libc",
        "//cargo/vendor/nix-0.15.0:nix",
        "//cargo/vendor/rayon-1.3.0:rayon",
        "//cargo/vendor/serde-1.0.104:serde",
        "//cargo/vendor/serde-bench-0.0.7:serde_bench",
        "//cargo/vendor/serde_bytes-0.11.3:serde_bytes",
        "//cargo/vendor/serde_derive-1.0.104:serde_derive",
        "//cargo/vendor/target-lexicon-0.9.0:target_lexicon",
        "//cargo/vendor/wasmer-clif-fork-frontend-0.52.0:wasmer_clif_fork_frontend",
        "//cargo/vendor/wasmer-clif-fork-wasm-0.52.0:wasmer_clif_fork_wasm",
        "//cargo/vendor/wasmer-runtime-core-0.13.1:wasmer_runtime_core",
        "//cargo/vendor/wasmparser-0.45.2:wasmparser",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.13.1",
    crate_features = [
    ],
)

