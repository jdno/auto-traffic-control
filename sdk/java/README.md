
<!-- The authors of auto-traffic-control release this file under either Apache v2.0 or MIT license terms. -->

# 🛬 Auto Traffic Control SDK - Java

[Homepage](https://auto-traffic-control.com) |
[Documentation](https://auto-traffic-control.com/docs) |
[Community](https://github.com/jdno/auto-traffic-control/discussions)

This project generates the Java SDK for the Auto Traffic Control game's gRPC API.
Until the maintainers of Auto Traffic Control publish official binaries to maven central, you can use this gradle script and the api proto files to generate a jar with grpc.

## Usage

This sdk utilizes gradle - v7.0 or later - to drive the protobuf compiler.

. `cd sdk/java`
. `gradle clean build publishToMavenLocal`

You can now add auto-traffic-control-sdk-java as a dependency in your maven or gradle script:

```
// gradle terse
implementation 'dev.jdno.atc:auto-traffic-control-sdk-java:0.3.2

// gradle verbose
implementation group:'dev.jdno.atc' name: 'auto-traffic-control-sdk-java' version: '0.3.2'

// maven
<dependency>
	<groupId>
		dev.jdno.atc
		</groupId>
	<artifactId>
		auto-traffic-control-sdk-java
		</artifactId>
	<version>
		0.3.2
		</version>
	</dependency>
```

Check out the [documentation](https://auto-traffic-control.com) to learn about the different services and their endpoints.
Consider studying the [example project](../../examples/java/src/main/java/com/example).



