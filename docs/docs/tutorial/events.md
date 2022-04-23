---
sidebar_position: 6
---

# Receiving Events

**Auto Traffic Control's** API is heavily inspired by
the [CQRS + Event Sourcing](https://danielwhittaker.me/2020/02/20/cqrs-step-step-guide-flow-typical-application/)
software patterns. The game streams _events_ to players, who can respond by
sending _commands_ to the game. You can find a list of the
[game's events](/docs/api/events) in the [API documentation](/docs/api).

## Events in TypeScript

Before diving into the implementation, let's take a brief moment to familiarize
ourselves with the data that we'll be dealing with.

Our goal for this chapter is to subscribe to the game's event stream. The stream
is provided by the [`EventService`] through its `stream` method. If we check the
[documentation], we can find this method signature:

```protobuf
rpc Stream(StreamRequest) returns (stream StreamResponse) {}
```

What this tells us is that when we subscribe to the event stream by calling the
`stream` method on the [`EventService`], we get back an iterator that yields
`StreamResponse` objects. The definition of those can also be found in the
[documentation], or alternatively directly in the [API specification]:

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

The `oneof` key is similar to an `enum`, and means that the response will
contain one of these events. Depending on the programming language, the
generated code might be an actual `enum` or something a little bit more
complicated.

:::tip You can probably inspect the generated code inside
the `auto-traffic-control`
package using your editor or IDE. This can be helpful to understand the
internals of the SDK and how to work with the different data types.
:::

In [TypeScript], it's a little bit more complicated. If we look inside the
package for the generated code, we can see that `StreamResponse` is an object
with fields for each variant. It also has an `enum` called `EventCase` that lets
us know what event we are dealing with. But [gRPC] sadly does not allow us to
work with the _names_ of the enum variants, only with their integer values.

We'll therefore take a different approach and work with the `StreamResponse`
object and not the `EventCase`.

## Processing a `StreamResponse`

Let's start by creating a function that will process each message in the event
stream. The function takes a `StreamResponse` object as its parameter, and it
does not return anything:

```typescript
function processMessage(streamResponse: StreamResponse): void {}
```

We have two goals for now:

1. Detect when an airplane is spawned and needs a new flight plan
2. End the program when the game ends

We can check if one of these events happened by testing whether the
corresponding field has been set on the `StreamResponse` object:

```typescript
function processMessage(streamResponse: StreamResponse): void {
  const airplaneDetected = streamResponse.getAirplaneDetected();
  if (airplaneDetected != undefined) {
    updateFlightPlan(airplaneDetected);
  }

  const gameStopped = streamResponse.getGameStopped();
  if (gameStopped != undefined) {
    exit(gameStopped);
  }
}
```

### Exiting the program

When the game ends, i.e. we receive the `GameStopped` event, we want to print
the score and exit the program. Let's create a new function that takes the
`GameStopped` event as a parameter, prints the score from the event, and then
exits:

```typescript
function processMessage(streamResponse: StreamResponse): void {
  // airplane detected

  const gameStopped = streamResponse.getGameStopped();
  if (gameStopped != undefined) {
    exit(gameStopped);
  }
}

function exit(event: GameStopped): void {
  const score = event.getScore();

  console.log(`Game stopped! Score: ${score}`);
  process.exit();
}
```

### Creating a flight plan

The second event that we are listening for is the `AirplaneDetected` event. When
this event occurs, a new airplane has entered the map. Airplanes are spawned
with a random flight plan, which means they'll wander around the map aimlessly.

We want to generate a new flight plan for the aircraft, which requires the
following information:

- The tag (i.e. color) of the airplane
- The first node in the airplane's current flight plan
- The location of the airport with the matching tag

The first two can be retrieved from the `AirplaneDetected` event, while the last
requires an API request to the [`MapService`](/docs/api/Services/map-service).

I'll leave the implementation of this as a challenge for the reader. The
following snippet simply prints a message including the airplane's ID and next
destination.

```typescript
function processMessage(streamResponse: StreamResponse): void {
  const airplaneDetected = streamResponse.getAirplaneDetected();
  if (airplaneDetected != undefined) {
    updateFlightPlan(airplaneDetected);
  }

  // game stopped
}

function updateFlightPlan(event: AirplaneDetected): void {
  const airplane = event.getAirplane();
  if (airplane == undefined) {
    throw new Error("Received AirplaneDetected event without an airplane");
  }

  const id = airplane.getId();
  const flightPlan = airplane.getFlightPlanList();
  const nextNode = flightPlan.at(0);

  console.log(`Detected airplane ${id} heading towards ${nextNode}.`);
}
```

## Subscribing to events

Let's create a new function that subscribes the program to the game's event
stream. We start as we usually do by initializing a client for the
[`EventService`], and then calling its `stream` endpoint:

```typescript
function subscribeToEvents(): void {
  const eventService = new EventServiceClient(
    "localhost:4747",
    getCredentials()
  );

  const stream = eventService.stream(new StreamRequest());
}
```

The `stream` method returns a `ClientReadableStream` object. We can register an
event listener on this object that gets called every time a new message is
received:

```typescript
stream.on("data", processMessage);
```

We also want to gracefully handle the end of the stream, e.g. when the game gets
quit. We can do this by listening for the `end` event on the stream object:

```typescript
stream.on("end", streamClosed);
```

Combining everything, this is how our function looks in the end:

```typescript
function subscribeToEvents(): void {
  const eventService = new EventServiceClient(
    "localhost:4747",
    getCredentials()
  );

  const stream = eventService.stream(new StreamRequest());

  stream.on("data", processMessage);
  stream.on("end", streamClosed);
}
```

## Playing a game

We can now put everything together. Let's take our previous `main` function, and
extract its content into a new function:

```typescript
function startGame(): void {
  const gameService = new GameServiceClient("localhost:4747", getCredentials());

  gameService.startGame(new StartGameRequest(), (err) => {
    if (err != null) {
      throw err;
    }

    console.log("Started a new game. Good luck!");
  });
}
```

We can now update the `main` function to first subscribe to the event stream,
and then start a new game:

```typescript
function main() {
  subscribeToEvents();
  startGame();
}
```

Fire up the **itch** app and launch **Auto Traffic Control**. Then start your
program with `npm start`, and watch the game. Airplanes will start to spawn
around the map, and the program will print their `id` and next node to the
terminal.

```shell
$ npm start

> node-traffic-controller@0.0.0 start
> npx ts-node src/main.ts

Started a new game. Good luck!
Detected airplane AT-0001 heading towards 11,-6.
Detected airplane AT-0002 heading towards -11,-3.
Detected airplane AT-0003 heading towards 4,8.
Detected airplane AT-0004 heading towards 11,3.
Detected airplane AT-0005 heading towards 5,-8.
Detected airplane AT-0006 heading towards 11,-7.
Detected airplane AT-0007 heading towards -11,1.
Detected airplane AT-0008 heading towards -2,-8.
Game stopped! Score: 0
```

Congrats! 🎉 You now know the basics of how to play the game.

## Choosing your own adventure

There is still a lot to discover and figure out, but you've learned the basics
of **Auto Traffic Control** and its API. Most importantly, you know how to start
a game and watch the event stream for changes in the game world. This is a great
place from which to start exploring!

The next step should probably be figuring out how to generate a flight plan for
an airplane. As mentioned earlier, this requires that you know not only the
current position of the airplane but also its destination. You can use the
[`MapService`](/docs/api/Services/map-service) to query the
[`Map`](/docs/api/types#map) and find out where the airports are.

Once you know where you are and where you want to go, it's time to figure out
the path. You can either implement your own path finding algorithm or look for a
library. Popular path finding algorithms for games are
[Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) and
[A\*](https://en.wikipedia.org/wiki/A*_search_algorithm), and it can be great fun
to implement them yourself. Or you can search [npm](https://www.npmjs.com/) for
a library that you can just plug into your program.

And path finding is really just the beginning...

[api specification]: https://github.com/jdno/auto-traffic-control/tree/main/api
[grpc]: https://grpc.io
[`eventservice`]: /docs/api/Services/event-service
[documentation]: /docs/api/Services/event-service
[typescript]: https://www.typescriptlang.org
