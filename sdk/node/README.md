# 🛬 Auto Traffic Control

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/jdno/auto-traffic-control/main)](https://github.com/jdno/auto-traffic-control/actions)
[![Version](https://img.shields.io/npm/v/auto-traffic-control)](https://crates.io/crates/auto-traffic-control)
[![License](https://img.shields.io/crates/l/auto-traffic-control)](https://crates.io/crates/auto-traffic-control)

[Homepage](https://auto-traffic-control.com) |
[Documentation](https://auto-traffic-control.com/docs) |
[Community](https://github.com/jdno/auto-traffic-control/discussions)

**Auto Traffic Control** is a video game played by programming. The player's
task is to create a program that can safely manage the airspace above two
airports.

The game is designed to provide an open-ended sandbox that players can use to
freely practice programming. The game provides a language-agnostic gRPC API,
giving players free choice of programming language or paradigm.

This crate contains the auto-generated Node SDK for the game's gRPC API.

## Usage

First, install `auto-traffic-control` as a new dependency:

```shell
npm install auto-traffic-control
```

Then, create a service client and send a request. Check out the
[documentation](https://auto-traffic-control.com) to learn about the different
services and their endpoints.

The following TypeScript [example](../../examples/typescript/main.ts) queries
the version of the game through the
[`AtcService`](https://auto-traffic-control.com/docs/api/Services/atc-service).

<!-- markdownlint-disable line-length -->

```typescript
import {
  getCredentials,
  AtcServiceClient,
  GetVersionRequest,
  GetVersionResponse,
} from "auto-traffic-control";

const atcService = new AtcServiceClient("localhost:4747", getCredentials());

atcService.getVersion(
  new GetVersionRequest(),
  (err, response: GetVersionResponse) => {
    if (err != null) {
      throw err;
    }

    const version = response.getVersion();

    if (version != null) {
      let versionString = [
        version.getMajor(),
        version.getMinor(),
        version.getPatch(),
      ].join(".");

      if (version.getPre() !== "") {
        versionString = versionString.concat(version.getPre());
      }

      console.log(`Auto Traffic Control is running version '${versionString}'`);
    } else {
      throw new Error("Requesting the version returned an empty response.");
    }
  }
);
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
