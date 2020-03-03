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



rust_library(
    name = "cranelift_native",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/cranelift-codegen-0.52.0:cranelift_codegen",
        "//cargo/vendor/raw-cpuid-7.0.3:raw_cpuid",
        "//cargo/vendor/target-lexicon-0.9.0:target_lexicon",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.52.0",
    crate_features = [
        "default",
        "std",
    ],
)

