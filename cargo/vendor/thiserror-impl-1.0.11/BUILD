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
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "thiserror_impl",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/proc-macro2-1.0.9:proc_macro2",
        "//cargo/vendor/quote-1.0.2:quote",
        "//cargo/vendor/syn-1.0.16:syn",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.0.11",
    crate_features = [
    ],
)

