# Examples

## [embedded-no-std](embedded-no-std.rs)

This shows how it could be used with a no-std allocator. Given an allocator, actually nothing is different from std usage.

## [library](library.rs)

This demonstrates how libraries could use the error type, while still providing discoverable recovery information and good error messages and interfacing.

## [non-dev-user](non-dev-user.rs)

This presents how messages for non-developer users could be constructed without losing information for developers debugging the thing.

## [tool-cli](tool-cli.rs)

Demonstration of interactions with `ExitCode` and general bubbling up.

## [validation](validation.rs)

An example of stacking multiple validation errors together ergonomically.

## [web-client](web-client.rs)

Recovering from retryable errors in a web client application.

## [webserver-backend](webserver-backend.rs)

Integration into common webserver backends to provide adequate automatic error responses.
