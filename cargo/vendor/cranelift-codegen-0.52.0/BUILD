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

rust_binary(
    name = "cranelift_codegen_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2018",
    deps = [
        "//cargo/vendor/cranelift-codegen-meta-0.52.0:cranelift_codegen_meta",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "basic-blocks",
      "default",
      "std",
    ],
    data = glob(["*"]),
    version = "0.52.0",
    visibility = ["//visibility:private"],
)

genrule(
    name = "cranelift_codegen_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["cranelift_codegen_out_dir_outputs.tar.gz"],
    tools = [
      ":cranelift_codegen_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/cranelift_codegen_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_BASIC_BLOCKS=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_STD=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/cranelift_codegen_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :cranelift_codegen_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)


rust_library(
    name = "cranelift_codegen",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/byteorder-1.3.4:byteorder",
        "//cargo/vendor/cranelift-bforest-0.52.0:cranelift_bforest",
        "//cargo/vendor/cranelift-codegen-shared-0.52.0:cranelift_codegen_shared",
        "//cargo/vendor/cranelift-entity-0.52.0:cranelift_entity",
        "//cargo/vendor/log-0.4.8:log",
        "//cargo/vendor/smallvec-1.2.0:smallvec",
        "//cargo/vendor/target-lexicon-0.9.0:target_lexicon",
        "//cargo/vendor/thiserror-1.0.11:thiserror",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":cranelift_codegen_build_script_executor",
    version = "0.52.0",
    crate_features = [
        "basic-blocks",
        "default",
        "std",
    ],
)

