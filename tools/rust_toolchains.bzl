load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")

def rust_toolchain_repositories():
    rust_repository_set(
        name = "rust_nightly_linux_x86_64",
        exec_triple = "x86_64-unknown-linux-gnu",
        extra_target_triples = [],
        version = "nightly",
        iso_date = "2020-01-30",
    )