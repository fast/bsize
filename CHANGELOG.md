# CHANGELOG

All significant changes to this software be documented in this file.

## Unreleased

### Breaking changes

* The trait bound `Unsigned` has been renamed to `ByteSize`.

### New features

* Added a new trait bound `Displayable`, which is implemented for all unsigned integer types that implement `ByteSize`.
* Added a new constructor method `bsize::display` that accepts `impl Displayable` and returns a `Display` instance.

## v0.1.0 (2026-06-16)

* Initial release.
