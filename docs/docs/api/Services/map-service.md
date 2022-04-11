---
title: Map
---

# `MapService`

The `MapService` can be queried to get the map of the current game session, or
to convert between the different coordinate systems.

## Get Map

The map used in the current game session can be retrieved using the `GetMap`
endpoint.

```protobuf
rpc GetMap(GetMapRequest) returns (GetMapResponse);
```

### Request

```protobuf
message GetMapRequest {}
```

### Response

```protobuf
message GetMapResponse {
  Map map = 1;
}
```

## Node to Point

Nodes are a location on the routing grid, while points are a location on the
map. A [`Node`](/docs/api/types#node) can be converted to a
[`Point`](/docs/api/types#point) through the `NodeToPoint` endpoint, but points
cannot be converted to nodes.

```protobuf
rpc NodeToPoint(NodeToPointRequest) returns (NodeToPointResponse);
```

### Request

```protobuf
message NodeToPointRequest {
  Node node = 1;
}
```

### Response

```protobuf
message NodeToPointResponse {
  Point point = 1;
}
```
