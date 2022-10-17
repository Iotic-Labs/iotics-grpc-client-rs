# iotics-grpc-client-rs

[![Crates][crates_badge]][crates]
[![Crates.io][crates_installs_badge]][crates]
[![CI][ci_badge]][ci]
[![dependency status][dependencies_badge]][dependencies]
[![license][license_badge]][license]\
IOTICS gRPC client in Rust.

## Usage and Version Compatibility with Iotics host:

| iotics-grpc-client-rs | iotics-host |
|----------------------| ----------- |
|      `iotics-grpc-client = "2.0.0"`       | `>= 5`       |
|      `iotics-grpc-client = "0.3.0"`      | `>= 4`     |

[API Docs][api_docs]

## Running the examples

Rename `.env-sample` to `.env` and give values to the environment variables.

```bash
git clone --recurse-submodules git@github.com:Iotic-Labs/iotics-grpc-client-rs.git
cargo run --features tls --example search
```

## Contributing

### Proto files

- Are submoduled in [api/](api/) from [https://github.com/Iotic-Labs/api][api_remote].
- [proto/google/rpc/status.proto][google_proto_local] is included as it's a dependency.

### Incrementing the API version

Run

```bash
cd api
git fetch --all --tags
git checkout tags/vX.X.X
cd ..
git add api

git commit -m "Updated api to vX.X.X"
git push
```

### Updating the API version

```bash
git submodule update --remote
```

### PRs

Commands to run before creating a PR

```bash
cargo make format
cargo make clippy
```

Should contain a summary of the changes in [CHANGELOG.md](README.md) under the Unreleased section.

### Releasing

- Increment the version in [Cargo.toml][cargo_version]
- Update [README.md](README.md) and [CHANGELOG.md](CHANGELOG.md) as needed
- Commit
- Tag the commit and push the changes to `main`

```bash
    git push
    git tag -a vX.X.X -m "vX.X.X"
    git push origin vX.X.X
```

- Release\
  Create a [new release][releases] containing the relevant change log.\
  The [publish.yaml][publish_action] GitHub Action will pick it up and do the actual release to [crates.io][crates].

[crates_badge]: https://img.shields.io/crates/v/iotics-grpc-client.svg
[crates]: https://crates.io/crates/iotics-grpc-client
[crates_installs_badge]: https://img.shields.io/crates/d/iotics-grpc-client?label=cargo%20installs
[ci_badge]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/actions
[dependencies_badge]: https://deps.rs/repo/github/Iotic-Labs/iotics-grpc-client-rs/status.svg?style=flat-square
[dependencies]: https://deps.rs/repo/github/Iotic-Labs/iotics-grpc-client-rs
[license_badge]: https://img.shields.io/crates/l/iotics-grpc-client.svg
[license]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/blob/main/LICENSE
[api_docs]: https://docs.rs/iotics-grpc-client/latest
[api_remote]: https://github.com/Iotic-Labs/api
[google_proto_local]: proto/google/rpc/status.proto
[cargo_version]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/blob/main/Cargo.toml#L3
[releases]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/releases
[publish_action]: https://github.com/Iotic-Labs/iotics-grpc-client-rs/actions/workflows/security-audit.yml
