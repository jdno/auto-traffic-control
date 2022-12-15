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
