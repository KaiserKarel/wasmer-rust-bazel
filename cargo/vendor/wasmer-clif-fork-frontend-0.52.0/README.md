**This is a (hopefully short-lived) fork** of [Cranelift](https://github.com/CraneStation/cranelift) so that we can continue publishing Wasmer runtime related crates on crates.io. If you're reading this and want to use Cranelift, you almost certainly want to use [the version this is forked from](https://github.com/CraneStation/cranelift). If you're associated with Cranelift and would like to merge any changes we have, please reach out to us at engineering@wasmer.io. We'll be submitting pull requests, too.

NOTE: this is a fork of version `0.31` of Cranelift.

This crate provides a straightforward way to create a
[Cranelift](https://crates.io/crates/cranelift) IR function and fill it with
instructions translated from another language. It contains an SSA construction
module that provides convenient methods for translating non-SSA variables into
SSA Cranelift IR values via `use_var` and `def_var` calls.
