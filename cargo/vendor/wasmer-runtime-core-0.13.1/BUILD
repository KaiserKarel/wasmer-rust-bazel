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

rust_binary(
    name = "wasmer_runtime_core_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2018",
    deps = [
        "//cargo/vendor/blake2b_simd-0.5.10:blake2b_simd",
        "//cargo/vendor/cc-1.0.50:cc",
        "//cargo/vendor/rustc_version-0.2.3:rustc_version",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
    ],
    data = glob(["*"]),
    version = "0.13.1",
    visibility = ["//visibility:private"],
)

genrule(
    name = "wasmer_runtime_core_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["wasmer_runtime_core_out_dir_outputs.tar.gz"],
    tools = [
      ":wasmer_runtime_core_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/wasmer_runtime_core_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/wasmer_runtime_core_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :wasmer_runtime_core_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)


rust_library(
    name = "wasmer_runtime_core",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bincode-1.2.1:bincode",
        "//cargo/vendor/blake2b_simd-0.5.10:blake2b_simd",
        "//cargo/vendor/digest-0.8.1:digest",
        "//cargo/vendor/errno-0.2.4:errno",
        "//cargo/vendor/hex-0.3.2:hex",
        "//cargo/vendor/indexmap-1.3.2:indexmap",
        "//cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//cargo/vendor/libc-0.2.67:libc",
        "//cargo/vendor/nix-0.15.0:nix",
        "//cargo/vendor/page_size-0.4.2:page_size",
        "//cargo/vendor/parking_lot-0.9.0:parking_lot",
        "//cargo/vendor/serde-1.0.104:serde",
        "//cargo/vendor/serde-bench-0.0.7:serde_bench",
        "//cargo/vendor/serde_bytes-0.11.3:serde_bytes",
        "//cargo/vendor/serde_derive-1.0.104:serde_derive",
        "//cargo/vendor/smallvec-0.6.13:smallvec",
        "//cargo/vendor/wasmparser-0.45.2:wasmparser",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":wasmer_runtime_core_build_script_executor",
    version = "0.13.1",
    crate_features = [
    ],
)

