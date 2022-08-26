# 🛬 Auto Traffic Control

**Auto Traffic Control** is a video game played by programming. The player's
task is to create a program that can safely manage the airspace above two
airports.

This gem provides a Ruby client for the game's gRPC API.

## Installation

Add these lines to your application's Gemfile:

```ruby
gem 'atc'
gem 'grpc'
```

And then execute:

```shell
bundle install
```

Or install it yourself as:

```shell
gem install atc
```

## Usage

The following example shows how to request the game's version from the server.
Check the [API docs](https://auto-traffic-control.com/docs/api) to learn about
the other services and their endpoints.

```ruby
require "atc/v1/atc_services_pb"
require "grpc"

def main
  service = Atc::V1::AtcService::Stub.new("localhost:4747", :this_channel_is_insecure)

  version = service.get_version(Atc::V1::GetVersionRequest.new).version

  p "Auto Traffic Control is running version #{version.major}.#{version.minor}.#{version.patch}"
end
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
