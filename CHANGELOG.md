# CHANGELOG

All significant changes to this software be documented in this file.

## Unreleased

### Breaking changes

* The trait bound `Unsigned` has been renamed to `ByteSize`.

### New features

* Added a new trait bound `Displayable`, which is implemented for all unsigned integer types that implement `ByteSize`.
* Added a new constructor method `bsize::display` that accepts `impl Displayable` and returns a `Display` instance.
* Added `DisplayOptions`, `DisplayBaseUnit`, `DisplayScale`, and `DisplayUnitSystem` for customizing `Display` output.
* Added `Display::options` for applying custom display options.
* Added support for formatting displayed byte sizes as bits and with a fixed scale.
* Added support for standard formatter width, fill, and alignment options on `Display`.

### Changed

* `Display::options` now accepts a closure over the current `DisplayOptions`, allowing custom options to compose with the active binary or decimal preset.

## v0.1.0 (2026-06-16)

* Initial release.
