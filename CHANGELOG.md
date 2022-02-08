# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com](http://keepachangelog.com/).

## [Unreleased]

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
[v0.2.7]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.7
[v0.2.6]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.6
[v0.2.5]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.5
[v0.2.4]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.4
[v0.2.3]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.3
[v0.2.2]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.2
[v0.2.1]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/tree/v0.2.1
