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

## Features

* `#![no_std]`-capable, no dependencies, and uses no heap allocation.
* `BSize` wrappers over `u8`, `u16`, `u32`, `u64`, and `usize` for representing byte sizes with different underlying types.
* `FromStr` impl for `BSize`, allowing for parsing string size representations like "1.5 KiB" and "521 TB".
* `Display` impl for `BSize`, allowing for formatting byte sizes as human-readable strings in both binary (e.g., "1.5 MiB") and decimal (e.g., "1.5 MB") styles.
* Optional `serde` support for binary and human-readable format.

## Documentation

Read the online documents at https://docs.rs/bsize.

## Why Yet Another Byte Size Crate?

There are already several crates that provide functionality for parsing, formatting, and/or representing byte sizes.

A new crate would always be doubted as nothing more than another competing standard.

[![Competing Standards](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)

This section shares the rationale behind this crate and how it differs from existing ones.

### `humansize`

The most commonly used crate for formatting byte sizes is [`humansize`](https://crates.io/crates/humansize). It provides a `format_size`/`format_size_i` function that formats a byte size into a human-readable string.

This function works well. However, when you want to define a struct that represents a byte size, `humansize` does not provide a type for that.

I have a large set of code looking like this:

```rust
const BASE_BLOB_INDEX_SIZE: usize = 4 * 1024; // 4 KiB
const BASE_BLOCK_SIZE: usize = 16 * 1024 * 1024; // 16 MiB
const RESERVED_MEMORY: usize = 256 * 1024 * 1024; // 256 MiB
const RESULT_SIZE_LIMIT: usize = 8 * 1024 * 1024 * 1024; // 8 GiB
```

I want them to be:

```rust
const BASE_BLOB_INDEX_SIZE: BSize<usize> = BSize::kib(4);
const BASE_BLOCK_SIZE: BSize<usize> = BSize::mib(16);
const RESERVED_MEMORY: BSize<usize> = BSize::mib(256);
const RESULT_SIZE_LIMIT: BSize<usize> = BSize::gib(8);
```

So you don't have to multiply the numbers by hand and rely on comments to indicate the units. This also makes it easier to change the units later if needed.

What's more, when you want to parse a byte size from a string, `humansize` does not provide a function for that either.

You can read [this issue](https://github.com/fast/bsize/issues/3) for the design discussion around the `Display` implementation for `BSize`.

### `parse-size`

The [`parse-size`](https://crates.io/crates/parse-size) crate provides a `parse_size` function that parses a byte size from a string.

Similarly, when you need to define a struct that represents a byte size, or when you want to format a byte size into a human-readable string, `parse-size` does not provide functionalities for either of those.

Besides, `parse-size` supports parsing sizes that has an exponential notation, such as `1e6` for 1 million bytes. This crate does not support that in the `FromStr::from_str` implementation, as it is not a common way to represent byte sizes. If it turns out to be useful, this crate may add a standalone function for that in the future.

### `bytesize`

The [`bytesize`](https://crates.io/crates/bytesize) crate provides a `ByteSize` struct that represents a byte size and implements `Display` and `FromStr` for it.

I was more than happy to try `bytesize` at first. However, I found that it does not provide a way to specify the underlying integer type for the byte size. It uses `u64` internally, while most of the constants shown above are of type `usize`. This means that I have to convert between `u64` and `usize` frequently, which is not ideal. See [this issue](https://github.com/bytesize-rs/bytesize/issues/135) for more details.

What's more, to support calculations between `BSize` and numeric types, this crate implements `BSize::map` for producing a new `BSize`, and `BSize::with` for producing an arbitrary result from the underlying byte count. This avoids implementing arithmetic traits for calculations between `BSize` and numeric types. The latter would cause confusions like what result type should be used for `ByteSize + u64`. However, `BSize` implements arithmetic traits for calculations between `BSize` and `BSize`, which is more intuitive and less error-prone.

```rust
let result = ByteSize::kib(4) + 64; // Is the result type ByteSize or u64? Why?
let result = BSize::<u64>::kib(4).map(|b| b + 64); // Clearly the result type is BSize.
let result = BSize::<u64>::kib(4).0 + 64; // Clearly the result type is u64.
let result = BSize::<u64>::kib(4).with(|b| b + 64); // when .0 is cumbersome sometimes, this is more convenient.
```

There is no `Unit` as well. To obtain a constant for a specific unit, you can use `BSize::<u64>::kib(1).0` and this can be resolved at compile time.

Finally, the following issues in `bytesize` have been resolved in this crate:

* [Unit measurements should not convert from XiB to XB](https://github.com/bytesize-rs/bytesize/issues/16): In `bsize`, the default Display implementation uses `B` always. This forces the user to customize the formatting if they want by calling `BSize::display`, and thus it reduces confusion on APIs.
* [Support for no-alloc environments](https://github.com/bytesize-rs/bytesize/issues/140): In `bsize`, both parsing and formatting functionalities are available in no-alloc environments.

## Minimum Rust Version Policy

This crate's minimum supported `rustc` version is `1.85.0`.

The current policy is that the minimum Rust version required to use this crate can be increased in minor version updates. For example, if `crate 1.0` requires Rust 1.85.0, then `crate 1.0.z` for all values of `z` will also require Rust 1.85.0 or newer. However, `crate 1.y` for `y > 0` may require a newer minimum version of Rust.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE).
