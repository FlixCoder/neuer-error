# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2026-01-31

- **breaking**: Added a marker generic to `NeuErr` to allow enforcing compile-time errors when no `.context` is given.
- **breaking**: Renamed `CtxResultExt` trait to `NeuErrResultExt`.

The `NeuErr` type without marker and the default `Result` will work as previously, not requiring added context. However, there is a new option to use a specific marker to require additional context before returning from the current function. This is simplified by a special macro `require_context!`. Please see the docs for more details.

Due to the marker changes, the `.context` methods will now return an error with the marker applied, so there might be conversions required in cases where the result was directly returned instead of using the question mark operator.

## [0.2.1] - 2026-01-24

- Documentation improvements only.

## [0.2.0] - 2026-01-15

- **breaking**: Renamed `CtxError`/`CtxErrorImpl` to `NeuErr`/`NeuErrImpl`.
- Implemented colored error formatting (with feature `colors`).

## [0.1.0] - 2026-01-11

- Initial release
