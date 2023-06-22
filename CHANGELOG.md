# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com](http://keepachangelog.com/).

## [Unreleased]
- Updated to IOTICS API v0.0.19

## [v.4.0.0] - 2023-01-20
- rename allowlist allhost/nohost value
## [v3.1.2] - 2023-01-06
- added missing no host property
## [v3.1.1] - 2023-01-06
- added missing host metadata allow list property

## [v3.1.0] - 2023-01-03
- removed visibility from upsert and update twin methods
## [v3.0.0] - 2022-12-23

### Added

- `create_channel` replaces all the `create_XXX_api_client` helper functions. It allows the configuration of `concurrency_limit`, `rate_limit` and `keep_alive_interval`.

### Changed

- `iotics_grpc_client::auth_builder::IntoAuthBuilder` has been moved to `iotics_grpc_client::IntoAuthBuilder`.
- The properties inside `iotics_grpc_client::common*` have been moved to `iotics_grpc_client`.

### Removed

- `create_XXX_api_client` helper functions have been removed. `create_channel` should be used instead.
- Remove of createdAt and updatedAt properties.

## [v2.0.1] - 2022-10-31

### Added

- `create_search_api_client`, `search`, `create_interest_api_client` and `follow` now take an extra `keep_alive_interval` parameter that allows the caller to set a KA interval for the underlying gRPC connection.

### Changed

- Updated to IOTICS API v0.0.16.

## [v2.0.0] - 2022-10-17

### Changed

- BREAKING CHANGE - Update to handle breaking changes in Iotics API, major change here is the change of TwinID Protobuf definition to include hostId (HostTwinDID)

## [v0.3.0] - 2022-08-30

### Added

- Search example.

### Changed

- TLS is now an optional feature which needs to be enabled explicitly.
- Updated to IOTICS API v0.0.12.


## [v0.2.7] - 2022-02-08

### Changed

- Update to the richer `list_all_twins` endpoint which is available starting with host version 3.0.722.

## [v0.2.6] - 2022-02-02

### Changed

- Errors bubbled up from host requests have the transaction references included.

## [v0.2.5] - 2022-01-31

### Changed

- `list_all_twins` has been reworked to perform the pagination internally.

## [v0.2.4] - 2022-01-27

### Changed

- `list_all_twins` and `list_all_twins_with_client` now take `limit` and `offset` parameters.

## [v0.2.3] - 2022-01-19

### Added

- Added connector common keys

## [v0.2.2] - 2022-01-18

### Fixed

- docs.rs build should now succeed

## [v0.2.1] - 2022-01-18

### Initial release of iotics-grpc-client

[unreleased]: https://github.com/Iotic-Labs/iotics-grpc-client-rs
[v4.0.0]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v4.0.0
[v2.0.1]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v2.0.1
[v2.0.0]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v2.0.0
[v0.3.0]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.3.0
[v0.2.7]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.7
[v0.2.6]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.6
[v0.2.5]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.5
[v0.2.4]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.4
[v0.2.3]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.3
[v0.2.2]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.2
[v0.2.1]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.1
