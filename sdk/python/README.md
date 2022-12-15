# 🛬 Auto Traffic Control

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/jdno/auto-traffic-control/main)](https://github.com/jdno/auto-traffic-control/actions)
[![License](https://img.shields.io/crates/l/auto-traffic-control)](https://www.npmjs.com/package/auto-traffic-control)

<!-- ![Version](https://img.shields.io/npm/v/auto-traffic-control)](https://www.npmjs.com/package/auto-traffic-control) -->

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
pip install auto-traffic-control
```

Or install it locally by cloning the source repo and running:

```shell
cd sdk/python
python -m pip install .
```

If build fails, try to upgrade your `pip` and try again:
`python -m pip install -U pip`.

Then, create a service client and send a request. Check out the
[documentation](https://auto-traffic-control.com) to learn about the different
services and their endpoints.

The following Python [example](../../examples/python/play.py) queries
the version of the game through the
[`AtcService`](https://auto-traffic-control.com/docs/api/Services/atc-service).

<!-- markdownlint-disable line-length -->

```python
import atc
import grpc


class ATCGame:

    BREAK = object()
    """Sentinel to break event loop"""

    def __init__(self, channel: grpc.Channel):
        self.channel = channel
        self.map: atc.Map = None

    def start(self):
        game = atc.GameServiceStub(self.channel)
        assert game.StartGame(atc.StartGameRequest()), "Failed to start game"

        # Subscribe to events and loop until someone returns BREAK
        events = atc.EventServiceStub(self.channel)
        for event in events.Stream(atc.StreamRequest()):
            event_name = event.WhichOneof("event")
            fn_name = f"on_{event_name}"
            if fn := getattr(self, fn_name):
                typed_event = getattr(event, event_name)
                if self.BREAK is fn(typed_event):
                    print(f"Received BREAK response from {fn_name}()")
                    break
            else:
                raise RuntimeError(
                    f"Event '{event_name}' does not have a handler function"
                )

    def on_game_started(self, event: atc.GameStarted):
        print("Game Started!")
        self.map = event.map

        # This is only here to show how to exit the game.
        # Remove this to continue receiving events.
        return self.BREAK

    def on_game_stopped(self, event: atc.GameStopped):
        print(f"Game Over: {event.score}")
        return self.BREAK

    def on_airplane_collided(self, event: atc.AirplaneCollided):
        raise RuntimeError(f"Collision detected: {event}")

    def on_airplane_detected(self, event: atc.AirplaneDetected):
        pass

    def on_airplane_landed(self, event: atc.AirplaneLanded):
        pass

    def on_airplane_moved(self, event: atc.AirplaneMoved):
        pass

    def on_flight_plan_updated(self, event: atc.FlightPlanUpdated):
        pass

    def on_landing_aborted(self, event: atc.LandingAborted):
        pass


def main():
    with grpc.insecure_channel("localhost:4747") as channel:
        svc = atc.AtcServiceStub(channel)
        resp: atc.GetVersionResponse = svc.GetVersion(atc.GetVersionRequest())
        print(
            f"Server version {resp.version.major}.{resp.version.minor}.{resp.version.patch}.{resp.version.pre}"
        )

        # Create the game instance and start
        game = ATCGame(channel)
        game.start()


if __name__ == "__main__":
    main()
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
