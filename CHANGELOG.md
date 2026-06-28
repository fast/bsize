# CHANGELOG

All significant changes to this software be documented in this file.

## Unreleased

### Breaking changes

* Renamed the old `BSize::with` mapping API to `BSize::map`.
* Removed the `Displayable` trait. `ByteSize` now has a `to_f64` method that is the same as the `Displayable::canonicalize` method.

### New features

* Added `nightly` feature for using `BSize` with nightly-only features like `const_ops` and `const_trait_impl`.
* Added a default `usize` underlying type for `BSize`, so `BSize` is equivalent to `BSize<usize>` in type positions.
* Added `BSize8`, `BSize16`, `BSize32`, and `BSize64` aliases.
* Added `ByteSize::to_f64` for converting supported byte size underlying types to approximate `f64` values.
* Added `BSize::as_b` for returning the byte count as an approximate `f64`.

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
