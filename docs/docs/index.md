---
title: Quickstart
---

# Quickstart

## 👋 Welcome

We're super happy that you're interested in our game.

The [Getting Started] guide shows you how to install the game, set up a client
library, and start playing.

If you are missing anything in the guide, let us know by opening an issue on
[GitHub].

## Install the Game

The recommended way to install the game is through the [itch.io] app, which runs
on Linux, macOS, and Windows. You will get updates for the game through the app
as well, making it super easy to play the best™ version of the game.

<!-- markdownlint-disable line-length no-inline-html -->

<iframe src="https://itch.io/embed/1463989?link_color=56a9de" width="552" height="167" frameborder="0"><a href="https://jdno.itch.io/auto-traffic-control">Auto Traffic Control by jdno</a></iframe>

<!-- markdownlint-enable line-length no-inline-html -->

## Set Up Your Program

When designing **Auto Traffic Control**, we wanted to give you maximum freedom
over your own code. Which means this getting started guide can only show you an
example that you can copy, tweak, or ignore. Whatever you choose to do, the goal
of this step is to set up a runnable program.

Below is an example for getting started with Rust.

### Rust

Creating an executable program in Rust is as simple as running `cargo`:

```shell
cargo new my-atc-program
```

You can then `cd` into the newly created directory and run the program with:

```shell
cargo run
```

## Download the Client Library

The game provides an API that you can use to query the state of the game and
issue commands. The API is built with [gRPC], and any programming language that
supports gRPC can be used to play the game.

We publish official client libraries for the following languages. Simply add the
package as a dependency to your project.

<!-- markdownlint-disable line-length -->

| Language | Package                                                                                                             |
| -------- | ------------------------------------------------------------------------------------------------------------------- |
| Node     | _coming soon_                                                                                                       |
| Rust     | [![Crates.io](https://img.shields.io/crates/v/auto-traffic-control)](https://crates.io/crates/auto-traffic-control) |

<!-- markdownlint-enable line-length -->

Missing your preferred programming language? [Open an issue on GitHub][github]
to let us know. Or use the [Protocol Buffers][proto-bufs] to generate your own
bindings.

## Connect to the Server

When you start the game, it runs a [gRPC] server in the background. You can
connect to the server at the following address:

```text
http://localhost:4747
```

How you create a gRPC client depends on the programming language and library
that you are using. Check out their documentation for instructions. In `Rust`,
it might look like this:

```rust
let mut game_service = GameServiceClient::connect("http://localhost:4747").await?;
```

## Start Playing

Now that you have an executable program, a client library for th [gRPC] API, and
a connection to the server, you can start playing the game by sending the
`StartGame` request.

Have fun exploring the game!

[getting started]: /docs
[github]: https://github.com/jdno/auto-traffic-control
[grpc]: https://grpc.io/
[itch.io]: https://itch.io
[proto-bufs]: https://github.com/jdno/auto-traffic-control/tree/main/api
