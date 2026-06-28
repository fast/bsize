# CHANGELOG

All significant changes to this software be documented in this file.

## Unreleased

## v0.3.0 (2026-06-28)

### Breaking changes

* Renamed the old `BSize::with` mapping API to `BSize::map`.
* Renamed the generic `BSize<T>` wrapper to `ByteSize<T>`. `BSize` is now an alias for `ByteSize<usize>`.
* Renamed the `ByteSize` trait to `BaseByteSize`.
* Removed the `Displayable` trait. `BaseByteSize` now has a `to_f64` method that is the same as the `Displayable::canonicalize` method.
* Made the inner `ByteSize` field private. Use `ByteSize::b` to construct byte sizes and `ByteSize::bytes` to get the exact underlying byte count.

### New features

* Added `nightly` feature for using `ByteSize` with nightly-only features like `const_ops` and `const_trait_impl`.
* Added `BSize8`, `BSize16`, `BSize32`, and `BSize64` aliases.
* Added `BaseByteSize::to_f64` for converting supported byte size base types to approximate `f64` values.
* Added `BSize::as_b` for returning the byte count as an approximate `f64`.
* Added support for formatting positive infinity with `Display::new`, which acts as an overflow marker.
* Added `ByteSize::bytes` for returning the exact byte count as the underlying integer type.

## v0.2.1 (2026-06-27)

### New features

* Added `Display::new` for formatting finite, non-negative `f64` byte counts directly.

## v0.2.0 (2026-06-27)

### Breaking changes

* The trait bound `Unsigned` has been renamed to `ByteSize`.

### New features

* Added a new trait bound `Displayable`, which is implemented for all unsigned integer types that implement `ByteSize`.
* Added a new top-level method `bsize::display` that accepts `impl Displayable` and returns a `Display` instance.
* Added `DisplayOptions`, `DisplayBaseUnit`, `DisplayScale`, and `DisplayUnitSystem` for customizing `Display` output.
* Added support for formatting displayed byte sizes as bits and with a fixed scale.
* Added support for standard formatter width, fill, and alignment options on `Display`.

## v0.1.0 (2026-06-16)

* Initial release.
