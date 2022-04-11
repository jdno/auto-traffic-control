---
title: Airplane
---

# `AirplaneService`

The `AirplaneService` implements the API endpoints that interact with airplanes
in the game. Most importantly, it can be used to update the flight plan of each
airplane.

## Get Airplane

The state of each airplane can be queried with the `GetAirplane` request. It
takes the `id` of the airplane as an argument, and returns an instance of the
[`Airplane`](/docs/api/types#airplane) message type.

```protobuf
rpc GetAirplane(GetAirplaneRequest) returns (GetAirplaneResponse);
```

### Request

```protobuf
message GetAirplaneRequest {
  string id = 1;
}
```

### Response

```protobuf
message GetAirplaneResponse {
  Airplane airplane = 1;
}
```

- The [status code] `NOT_FOUND` will be returned when no airplane with the given
  `id` can be found.

## Update Flight Plan

The flight plan of an airplane can be updated with the `UpdateFlightPlan`
request. The request takes the airplane's `id` and the new flight plan as an
argument, and returns either a success or error response.

```protobuf
rpc UpdateFlightPlan(UpdateFlightPlanRequest) returns (UpdateFlightPlanResponse);
```

### Request

```protobuf
message UpdateFlightPlanRequest {
  string id = 1;
  repeated Node flight_plan = 2;
}
```

### Response

```protobuf
message UpdateFlightPlanResponse {
  oneof payload {
    UpdateFlightPlanSuccess success = 1;
    UpdateFlightPlanError error = 2;
  }
}
```

When the flight plan is successfully updated, an empty response will be
returned.

```protobuf
message UpdateFlightPlanSuccess {}
```

When the flight plan cannot be updated, e.g. because the provided plan is not
valid, a list of validation errors is returned.

```protobuf
message UpdateFlightPlanError {
  enum ValidationError {
    UNSPECIFIED = 0;
    NODE_OUTSIDE_MAP = 1;
    INVALID_STEP = 2;
    SHARP_TURN = 3;
    INVALID_START = 4;
    RESTRICTED_NODE = 5;
  }
  repeated ValidationError errors = 1;
}
```

[status code]: https://grpc.github.io/grpc/core/md_doc_statuscodes.html
