---
sidebar_position: 2
---

# Events

Events are a core concept in **Auto Traffic Control**. Every change in the game
is streamed to players as an event. This page lists every available event type
and its payload.

## `AirplaneCollided`

When two airplanes get too close together, this event is triggered. The event
payload contains the ids of the two airplanes.

```protobuf
message AirplaneCollided {
  string id1 = 1;
  string id2 = 2;
}
```

## `AirplaneDetected`

The `AirplaneDetected` event is triggered every time that a new airplane is
spawned. All available information about the new airplane is included in the
[`Airplane`](/docs/api/types#airplane) message type in the event payload.

```protobuf
message AirplaneDetected {
  Airplane airplane = 1;
}
```

## `AirplaneLanded`

When an airplane lands, the `AirplaneLanded` event is triggered. The event
payload contains the `id` of the airplane.

:::caution
After landing, the airplane is despawned and cannot be queried anymore.
:::

```protobuf
message AirplaneLanded {
  string id = 1;
}
```

## `AirplaneMoved`

The `AirplaneMoved` event is triggered whenever an airplane moves on the map.
Since airplanes cannot stand still, this event gets send continuously for every
airplane in the game.

```protobuf
message AirplaneMoved {
  string id = 1;
  Point point = 2;
}
```

## `FlightPlanUpdated`

When an airplane gets a new flight plan, the `FlightPlanUpdated` event is sent.
Flight plans are usually updated by the player. But when an airplane reaches the
last node in its flight plane, the game will generate a random flight plan.

```protobuf
message FlightPlanUpdated {
  string id = 1;
  repeated Node flight_plan = 2;
}
```

## `LandingAborted`

The `LandingAborted` event is sent when an airplane attempts a landing on the
wrong airport. For example, a red airplane tries to land at the blue airport.

```protobuf
message LandingAborted {
  string id = 1;
}
```

## `GameStarted`

When a new game is started, the `GameStarted` event is triggered. The event
payload contains the map that was generated for this game session.

```protobuf
message GameStarted {
  Map map = 1;
}
```

## `GameStopped`

When the game ends, e.g. because two airplanes got too close together, the
`GameStopped` event is sent. The event payload contains the score that the
player achieved.

```protobuf
message GameStopped {
  uint32 score = 1;
}
```
