# 🛬 Auto Traffic Control

[Homepage](https://auto-traffic-control.com) |
[Documentation](https://auto-traffic-control.com/docs) |
[Community](https://github.com/jdno/auto-traffic-control/discussions)

**Auto Traffic Control** is a video game played by programming. The player's
task is to create a program that can safely manage the airspace above two
airports.

The game is designed to provide an open-ended sandbox that players can use to
freely practice programming. The game provides a language-agnostic gRPC API,
giving players free choice of programming language or paradigm.

This project generates the Java SDK for the game's gRPC API.

## Usage

This SDK utilizes gradle - v7.0 or later - to drive the protobuf compiler.

```shell
cd sdk/java
gradle clean build publishToMavenLocal
```

You can now add `auto-traffic-control` as a dependency in Gradle...

<!-- markdownlint-disable line-length -->

```groovy
// gradle terse
implementation 'com.auto-traffic-control:auto-traffic-control:0.3.2'

// gradle verbose
implementation group: 'com.auto-traffic-control' name : 'auto-traffic-control' version: '0.3.2'
```

<!-- markdownlint-enable line-length -->

...or Maven.

```xml
<dependency>
    <groupId>
        com.auto-traffic-control
    </groupId>
    <artifactId>
        auto-traffic-control
    </artifactId>
    <version>
        0.3.2
    </version>
</dependency>
```

Check out the [documentation](https://auto-traffic-control.com) to learn about
the different services and their endpoints, and see the
[example project](../../examples/java/src/main/java/com/example) for a reference
implementation.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT)
  or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
