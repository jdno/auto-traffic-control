---
title: Game
---

# `GameService`

The `GameService` can be used to start a new game or to query the status of the
current game session.

## Get Game

The state of the current game session can be queried with the `GetGame` request.

```protobuf
rpc GetGameState(GetGameStateRequest) returns (GetGameStateResponse);
```

### Request

```protobuf
message GetGameStateRequest {}
```

### Response

The response contains the `game_state`, which can be either `ready` or
`running`. When the game state is `ready`, no game session is currently running
and the player is either in the _main menu_ or in the _game over_ screen.

```protobuf
message GetGameStateResponse {
  enum GameState {
    GAME_STATE_UNSPECIFIED = 0;
    GAME_STATE_READY = 1;
    GAME_STATE_RUNNING = 2;
  }
  GameState game_state = 1;
}
```

## Start Game

A new game session can be started by calling the `StartGame` endpoint of the
`GameService`. If a game is already running, nothing will happen.

```protobuf
rpc StartGame(StartGameRequest) returns (StartGameResponse);
```

### Request

```protobuf
message StartGameRequest {}
```

### Response

```protobuf
message StartGameResponse {}
```
