# 🛬 Auto Traffic Control – Ruby Example

**Auto Traffic Control** is a video game played by programming. The player's
task is to create a program that can safely manage the airspace above two
airports.

This example shows how to connect with a Ruby client to the game's server, using
the official [Ruby gem](https://rubygems.org/gems/).

## Usage

First, change into the directory with the example:

```shell
cd examples/ruby
```

Then, install the dependencies:

```shell
bundle install
```

Finally, start the game and then run the example:

```shell
bundle exec rake run
```

## Development

For local development, you have to generate the Ruby bindings. This can most
easily be done with [Buf](https://buf.build/). Install the command-line tool,
create an account for remote execution, then generate a token and sign in to the
CLI. Check Buf's documentation for an up-to-date guide on this.

Then, change into the API directory and generate the Ruby bindings:

```shell
cd api
buf generate
```

After checking out the repo, run `bin/setup` to install dependencies. You can
also run `bin/console` for an interactive prompt that will allow you to
experiment.

To install this gem onto your local machine, run `bundle exec rake install`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
