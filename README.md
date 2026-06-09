# BSize

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MSRV 1.85][msrv-badge]](https://www.whatrustisit.com)
[![Apache 2.0 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/bsize.svg
[crates-url]: https://crates.io/crates/bsize
[docs-badge]: https://img.shields.io/docsrs/bsize
[docs-url]: https://docs.rs/bsize
[msrv-badge]: https://img.shields.io/badge/MSRV-1.85-green?logo=rust
[license-badge]: https://img.shields.io/crates/l/bsize
[license-url]: LICENSE
[actions-badge]: https://github.com/fast/bsize/workflows/CI/badge.svg
[actions-url]: https://github.com/fast/bsize/actions?query=workflow%3ACI

This crate provides multiple semantic wrappers and utilities for byte size representations.

## Minimum Rust version policy

This crate's minimum supported `rustc` version is `1.85.0`.

The current policy is that the minimum Rust version required to use this crate can be increased in minor version updates. For example, if `crate 1.0` requires Rust 1.85.0, then `crate 1.0.z` for all values of `z` will also require Rust 1.85.0 or newer. However, `crate 1.y` for `y > 0` may require a newer minimum version of Rust.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE).
