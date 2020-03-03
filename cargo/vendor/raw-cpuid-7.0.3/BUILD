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


# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "cache" with type "example" omitted
# Unsupported target "cpu" with type "example" omitted
rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_cpuid",
    crate_root = "src/bin/cpuid.rs",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their crate's lib
        ":raw_cpuid",
        "//cargo/vendor/bitflags-1.2.1:bitflags",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "7.0.3",
    crate_features = [
    ],
)


rust_library(
    name = "raw_cpuid",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-1.2.1:bitflags",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "7.0.3",
    crate_features = [
    ],
)

# Unsupported target "topology" with type "example" omitted
# Unsupported target "tsc_frequency" with type "example" omitted
