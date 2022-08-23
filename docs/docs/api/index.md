---
sidebar_position: 1
title: Introduction
---

# Introduction

**Auto Traffic Control** provides a [gRPC] API that players can use to interact
with the game. Official client libraries for popular programming languages are
generated from the [API specification](https://github.com/jdno/auto-traffic-control/tree/main/api).

<!-- markdownlint-disable line-length -->

| Language | Package                                                                                                              |
| -------- | -------------------------------------------------------------------------------------------------------------------- |
| Node     | [![npm](https://img.shields.io/npm/v/auto-traffic-control)](https://www.npmjs.com/package/auto-traffic-control)      |
| Rust     | [![Crates.io](https://img.shields.io/crates/v/auto-traffic-control)](https://crates.io/crates/auto-traffic-control)  |
| .Net     | [![Crates.io](https://img.shields.io/nuget/v/AutoTrafficControl)](https://www.nuget.org/packages/AutoTrafficControl) |

<!-- markdownlint-enable line-length -->

## Address

The API is served at the following location:

```text
http://localhost:4747
```

## Services

The API is organized into different services, each with its own area of
responsibility.

- The [`AirplaneService`](/docs/api/Services/airplane-service) provides
  information about the airplanes on the map, and allows updating their flight
  plans.
- The [`AtcService`](/docs/api/Services/atc-service) provides information about
  **Auto Traffic Control** itself, for example its current version.
- The [`EventService`](/docs/api/Services/event-service) streams every change in
  the game to the player.
- The [`GameService`](/docs/api/Services/game-service) can be used to start a
  new game.
- The [`MapService`](/docs/api/Services/map-service) provides information about
  the map and can convert between coordinate systems.

[grpc]: https://grpc.io/
