---
sidebar_position: 3
---

# Types

The API defines a set of message types for the resources in the game, which are
used in requests and responses.

## Airplane

Airplanes in Auto Traffic Control are uniquely identified by their `id`. The
`id` is passed as an argument to commands that can alter the state of an
airplane, e.g. update its flight plan.

The position of an airplane is provided as a [`Point`](/docs/api/types#point)
coordinate on the map.

The flight plan of an airplane is represented by a list of nodes. It can be
updated through the `AirplaneService`.

```protobuf
message Airplane {
  string id = 1;
  Point point = 2;
  repeated Node flight_plan = 3;
  Tag tag = 4;
}
```

## Airport

Airports have a location and a color.

The location of an airport is provided as a [`Node`](/docs/api/types#node)
coordinate, since it must be reachable via the routing grid.

```protobuf
message Airport {
  Node node = 1;
  Tag tag = 2;
}
```

## Map

The map type provides a full representation of the map that is used in the
current game session.

- The airports on the map are provided as a list of
  [`Airport`](/docs/api/types#airport) types.
- The routing grid consists of a list of [`Node`](/docs/api/types#node) types.

The routing grid can be indexed using the `width` and `height` of the map.

```protobuf
message Map {
  repeated Airport airports = 1;
  repeated Node routing_grid = 2;
  uint32 width = 3;
  uint32 height = 4;
}
```

## Node

Nodes represent locations on the routing grid.

Airplanes cannot move freely across the map, and instead must follow the routing
grid. Each reachable point in the routing grid is represented by a `Node`.

Nodes can be `restricted`, which means that airplanes are not allowed to pass
through it. Flight plans containing restricted nodes will be rejected by the
game.

The location of a node can be converted to a point on the map by calling the
`NodeToPoint` endpoint of the [`MapService`](/docs/api/Services/map-service).

```protobuf
message Node {
  int32 longitude = 1;
  int32 latitude = 2;
  bool restricted = 3;
}
```

## Point

A location on the map.

```protobuf
message Point {
  int32 x = 1;
  int32 y = 2;
}
```

## Tag

Tags match airplanes and airports, and are represented by a color.

Every airplane and airport has a tag. The tag of an airplane must match the tag
of an airport for it to land there.

```protobuf
enum Tag {
  TAG_UNSPECIFIED = 0;
  TAG_BLUE = 1;
  TAG_RED = 2;
}
```

## Version

The `AtcService` provides an endpoint that can be used to query the version of
the game. This can be useful to determine if the player's program is compatible
with the game.

**Auto Traffic Control** follows [semantic versioning](https://semver.org/), and
the version is returned as the combination of a `major`, `minor`, and `patch`
version. Pre-releases might also feature a `pre` label.

```protobuf
message Version {
  uint64 major = 1;
  uint64 minor = 2;
  uint64 patch = 3;
  string pre = 4;
}
```
