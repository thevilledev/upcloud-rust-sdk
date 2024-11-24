# upcloud-rust-sdk

[<img alt="crates.io" src="https://img.shields.io/crates/v/upcloud-sdk.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/upcloud-sdk)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-upcloud--sdk-8a2be2?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/upcloud-sdk)

Unofficial UpCloud API client for Rust.

## Features

- Strongly typed API with builder patterns for request construction
- Async/await support using Tokio
- Environment variable based configuration
- Trait-based design for resource operations

## Examples

See the following:

- [Simple example](examples/server_create_simple.rs): Create a server with a minimal set of options
- [Advanced example](examples/server_create_advanced.rs): Create a server with two interfaces and poll until server is started
- [List servers](examples/server_list.rs): List all servers and utilise label filtering

## Structure

- `src/client.rs`: Client implementation
- `src/resources/*`: Resource-specific code
- `src/types/*`: Shared types
- `src/constants.rs`: Constants
- `src/error.rs`: Error handling
- `src/config.rs`: Configuration

## TODO

- Add support to other resources than Cloud Servers
- Request/response serialisation/deserialisation improvements
- Error handling
- Documentation
- Release process

## License

MIT