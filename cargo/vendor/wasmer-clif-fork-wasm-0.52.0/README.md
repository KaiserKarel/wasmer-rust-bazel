**This is a (hopefully short-lived) fork** of [Cranelift](https://github.com/CraneStation/cranelift) so that we can continue publishing Wasmer runtime related crates on crates.io. If you're reading this and want to use Cranelift, you almost certainly want to use [the version this is forked from](https://github.com/CraneStation/cranelift). If you're associated with Cranelift and would like to merge any changes we have, please reach out to us at engineering@wasmer.io. We'll be submitting pull requests, too.

NOTE: this is a fork of version `0.31` of Cranelift.

This crate performs the translation from a wasm module in binary format to the
in-memory form of the [Cranelift IR].

If you're looking for a complete WebAssembly implementation that uses this
library, see [Wasmtime].

[Wasmtime]: https://github.com/bytecodealliance/wasmtime
[Cranelift IR]: https://cranelift.readthedocs.io/en/latest/ir.html
