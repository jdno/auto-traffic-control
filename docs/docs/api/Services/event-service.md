---
title: Event
---

# `EventService`

The `EventService` streams changes in the game as events to the player.

## Stream

Players can subscribe to the event stream by calling the `Stream` endpoint on
the `EventService`.

```protobuf
rpc Stream(StreamRequest) returns (stream StreamResponse) {}
```

### Request

```protobuf
message StreamRequest {}
```

### Response

The event service returns a stream of events. See [Events](/docs/api/events) for
more information on the different event types.

```protobuf
message StreamResponse {
  oneof event {
    AirplaneCollided airplane_collided = 1;
    AirplaneDetected airplane_detected = 2;
    AirplaneLanded airplane_landed = 3;
    AirplaneMoved airplane_moved = 4;
    FlightPlanUpdated flight_plan_updated = 5;
    LandingAborted landing_aborted = 6;
    GameStarted game_started = 7;
    GameStopped game_stopped = 8;
  }
}
```
