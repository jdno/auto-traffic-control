# 🛬 Auto Traffic Control

[![GitHub branch checks state](https://img.shields.io/github/checks-status/jdno/auto-traffic-control/main)](https://github.com/jdno/auto-traffic-control/actions)
[![Crates.io](https://img.shields.io/crates/v/auto-traffic-control)](https://crates.io/crates/auto-traffic-control)
[![Crates.io](https://img.shields.io/crates/l/auto-traffic-control)](https://crates.io/crates/auto-traffic-control)

[Homepage](https://auto-traffic-control.com) |
[Documentation](https://auto-traffic-control.com/docs/getting-started) |
[Community](https://github.com/jdno/auto-traffic-control/discussions)

**Auto Traffic Control** is a video game played by programming. The player's
task is to create a program that can safely manage the airspace above two
airports.

The game is designed to provide an open-ended sandbox that players can use to
freely practice programming. The game provides a language-agnostic gRPC API,
giving players free choice of programming language or paradigm.

This crate contains the auto-generated client for the gRPC API.

## Usage

First, add `auto-traffic-control` as a new dependency to your `Cargo.toml`.

```toml
[dependencies]
auto-traffic-control = "0.1.0"
```

You also need to add [`tonic`](https://crates.io/crates/tonic), the Rust
implementation of gRPC, and [`tokio`](https://crates.io/crates/tokio), the async
runtime, as a dependency.

Then, create a service client and send a request. Check out the
[documentation](https://auto-traffic-control.com) to learn about the different
services and their endpoints. The following [example](examples/get_version.rs)
queries the version of the game through the
[`AtcService`](https://auto-traffic-control.com/docs/api/Services/atc-service).

<!-- markdownlint-disable line-length -->

```rust
use auto_traffic_control::v1::atc_service_client::AtcServiceClient;
use auto_traffic_control::v1::GetVersionRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut atc_service = AtcServiceClient::connect("http://localhost:4747").await?;

    let response = atc_service.get_version(GetVersionRequest {}).await?;
    let version_field = response.into_inner().version;

    if let Some(version) = version_field {
        let mut version_string = format!("{}.{}.{}", version.major, version.minor, version.patch);

        if !version.pre.is_empty() {
            version_string.push('-');
            version_string.push_str(&version.pre);
        }

        println!("Auto Traffic Control is running version '{version_string}'");
    } else {
        println!("Requesting the version returned an empty response.");
    }

    Ok(())
}
```

<!-- markdownlint-enable line-length -->

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
