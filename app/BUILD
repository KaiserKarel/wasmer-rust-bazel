package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_binary(
    name = "example",
    edition = "2018",
    crate_type = "bin",
    srcs = [
        "src/main.rs",
    ],
    deps = [
        "//cargo:wasmer_runtime",
    ],
)
