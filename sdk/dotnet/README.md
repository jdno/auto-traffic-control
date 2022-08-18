# 🛬 Auto Traffic Control

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/jdno/auto-traffic-control/main)](https://github.com/jdno/auto-traffic-control/actions)
[![Version](https://img.shields.io/nuget/v/AutoTrafficControl)](https://www.npmjs.com/package/AutoTrafficControl)
[![License](https://img.shields.io/crates/l/auto-traffic-control)](https://www.npmjs.com/package/auto-traffic-control)

[Homepage](https://auto-traffic-control.com) |
[Documentation](https://auto-traffic-control.com/docs) |
[Community](https://github.com/jdno/auto-traffic-control/discussions)

**Auto Traffic Control** is a video game played by programming. The player's
task is to create a program that can safely manage the airspace above two
airports.

The game is designed to provide an open-ended sandbox that players can use to
freely practice programming. The game provides a language-agnostic gRPC API,
giving players free choice of programming language or paradigm.

This NuGet library contains the auto-generated .Net/C# SDK for the game's gRPC API.

## Usage

First, install `AutoTrafficControl` as a new dependency:

```shell
dotnet package add AutoTrafficControl
```

Then, create a service client and send a request. Check out the
[documentation](https://auto-traffic-control.com) to learn about the different
services and their endpoints.

The following C# [example](../../examples/dotnet/Program.cs) queries
the version of the game through the
[`AtcService`](https://auto-traffic-control.com/docs/api/Services/atc-service).

<!-- markdownlint-disable line-length -->

```csharp
using Atc.V1;
using Grpc.Core;
using Grpc.Net.Client;
using static Atc.V1.AtcService;

var channel = GrpcChannel.ForAddress("http://localhost:4747");
var atcClient = new AtcServiceClient(channel);

var response = await atcClient.GetVersionAsync(new GetVersionRequest());

var version = response?.Version;

if (version != null)
{
    var versionString = string.Join(".", new[] {
        version.Major,
        version.Minor,
        version.Patch
    });

    if (!string.IsNullOrEmpty(version.Pre))
    {
        versionString += version.Pre;
    }

    Console.WriteLine($"Auto Traffic Control is running version '{versionString}'");
}
else
{
    throw new InvalidOperationException("Requesting the version returned an empty response.");
}
```

If you want to use dependency injection to get clients, you can easily add all clients using the utility extension:

```csharp
services.AddAutoTrafficControlClients(new Uri("http://localhost:4747"));
```

At which point you can inject any client when creating your services:

```csharp
class AirplaneEventHandler {
  private readonly AirplaneServiceClient _client;

  public AirplaneEventHandler(AirplaneServiceClient client) {
    _client = client;
  }
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
