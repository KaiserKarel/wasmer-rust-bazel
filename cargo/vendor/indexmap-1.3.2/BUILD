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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_binary(
    name = "indexmap_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
        "//cargo/vendor/autocfg-1.0.0:autocfg",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "serde",
      "serde-1",
    ],
    data = glob(["*"]),
    version = "1.3.2",
    visibility = ["//visibility:private"],
)

genrule(
    name = "indexmap_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["indexmap_out_dir_outputs.tar.gz"],
    tools = [
      ":indexmap_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/indexmap_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_SERDE=1;"
        + " export CARGO_FEATURE_SERDE_1=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/indexmap_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :indexmap_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "bench" with type "bench" omitted
# Unsupported target "equivalent_trait" with type "test" omitted
# Unsupported target "faststring" with type "bench" omitted

rust_library(
    name = "indexmap",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/serde-1.0.104:serde",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":indexmap_build_script_executor",
    version = "1.3.2",
    crate_features = [
        "serde",
        "serde-1",
    ],
)

# Unsupported target "macros_full_path" with type "test" omitted
# Unsupported target "quick" with type "test" omitted
# Unsupported target "serde" with type "test" omitted
# Unsupported target "tests" with type "test" omitted
