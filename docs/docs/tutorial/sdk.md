---
sidebar_position: 4
---

# Getting the SDK

Now that we have a program that we can run, it's time to integrate the game's
API.

**Auto Traffic Control** uses the [gRPC] framework to provide a well-defined and
easy-to-use API to players. One feature of [gRPC] is that API clients can be
auto-generated from the [schema definitions] of the API. These clients hide the
internal details of [gRPC], and give you a high-level interface for interacting
with the game.

We publish these auto-generated client libraries for the most popular languages,
and call them our `software development kits` (SDKs).

:::tip
The [API documentation](/docs/api) provides an overview of the different data
types, services, and endpoints of the API.
:::

## Adding the npm package

**Auto Traffic Control's** SDK for the Node ecosystem is called available as a
npm package. Click the badge below to open the package on [npmjs.com]:

[![npm package](https://img.shields.io/npm/v/auto-traffic-control)](https://www.npmjs.com/package/auto-traffic-control)

As you can see on [npmjs.com], you can add the package as a dependency to your
project with the following command:

```shell
npm install auto-traffic-control
```

## Initializing the client

The game's API is divided into different _services_, which have their own area
of responsibility. For a quick proof-of-concept, we're going to call the
[`AtcService`] to
[query the version](/docs/api/Services/atc-service#get-version) of the game.

Open `src/main.ts`, and remove the following line from the `main` function:

```typescript
console.log("Hello, World!");
```

Let's start by initializing the [`AtcService`]. The SDK contains the
`AtcServiceClient`, which can be used to connect to the service. Pass in the
[address](/docs/api#address) of the game server, and call the helper method
`getCredentials` in the SDK to authenticate with the server.

```typescript
function main() {
  const atcService = new AtcServiceClient("localhost:4747", getCredentials());
}
```

Depending on your editor or IDE, you will get a `TS2304` error now, because the
names `AtcServiceClient` and `getCredentials` cannot be found. We have to import
them from the SDK first. Add the following at the top of `src/main.ts`:

```typescript
import { getCredentials, AtcServiceClient } from "auto-traffic-control";
```

If you run the program with `npm start` now, nothing will happen. You
initialized the client, but it only connects to the server when we call one of
its APIs. Let's do that next.

:::note
Authentication is a core functionality in [gRPC], and the client stubs require
credentials. You don't need to worry about this, just remember to get the
credentials from the SDK and pass them as the second argument when initializing
a client.
:::

## Getting the version

If you check the documentation of the [`AtcService`], you will see that it only
has a single API
endpoint: [`getVersion`](/docs/api/Services/atc-service#get-version).
You can actually inspect the generated code inside the SDK to see the full
method definition:

<!-- markdownlint-disable line-length -->

```typescript
public getVersion(
    request: GetVersionRequest,
    callback: (error: grpc.ServiceError | null, response: GetVersionResponse) => void
): grpc.ClientUnaryCall;
```

<!-- markdownlint-enable line-length -->

As you can see, this method takes a
[`GetVersionRequest`](/docs/api/Services/atc-service#request) as its first
parameter. The second parameter is a callback function that is passed either a
`ServiceError` or
a [`GetVersionResponse`](/docs/api/Services/atc-service#response).

Let's set up the API call with an empty callback method:

```typescript
function main() {
  const atcService = new AtcServiceClient("localhost:4747", getCredentials());

  atcService.getVersion(
    new GetVersionRequest(),
    (err: ServiceError | null, response: GetVersionResponse) => {}
  );
}
```

If your editor or IDE is not already telling you, don't forget to add
`ServiceError` and `GetVersionResponse` to the imports again.

### Handling errors

The callback function is passed a `ServiceError` when the API request failed.
For example, when the game is not running and the program cannot connect to the
game server. If that happens, we want to print the error and abort the program.
Luckily, we can simply throw an exception to achieve this:

```typescript
(err: ServiceError | null, response: GetVersionResponse) => {
  if (err != null) {
    throw err;
  }
};
```

If you run the program with `npm start` now (without having started the game),
you will get the following error:

```text
Error: 14 UNAVAILABLE: No connection established
```

Throwing an exception is not a pretty way to handle the issue, but it's good
enough for now.

### Unpacking the response

If the API request succeeds, the callback gets passed a
[`GetVersionResponse`](/docs/api/Services/atc-service#response). If you check
the [documentation](/docs/api/Services/atc-service#response), you will see that
the response has a single field called `version`. The field has the type
[`Version`](/docs/api/types#version), which has its own entry in the
[documentation](/docs/api/types#version) and looks like this:

```protobuf
message Version {
  uint64 major = 1;
  uint64 minor = 2;
  uint64 patch = 3;
  string pre = 4;
}
```

A slightly annoying
but [sensible decision](https://stackoverflow.com/a/31814967)
in [gRPC] is that fields can be uninitialized, in which case they are set to a
default value. For the `version` field, this means that it can be `undefined`.
So let's start by getting the version from the
[`GetVersionResponse`](/docs/api/Services/atc-service#response), and then check
if it's set:

```typescript
const version = response.getVersion();

if (version != null) {
  // TODO
} else {
  throw new Error("Requesting the version returned an empty response.");
}
```

I decided to simply throw an exception again, since the program cannot really do
anything interesting if the version is empty.

:::info
The game always sets every field in a response and an uninitialized value is
considered a bug.
:::

We can now access the fields of the `version` and merge them into a single
string that we'll print:

```typescript
if (version != null) {
  let versionString = [
    version.getMajor(),
    version.getMinor(),
    version.getPatch(),
  ].join(".");

  if (version.getPre() !== "") {
    versionString = versionString.concat(version.getPre());
  }

  console.log(`Auto Traffic Control is running version '${versionString}'`);
} else {
  throw new Error("Requesting the version returned an empty response.");
}
```

## Calling the API

Let's put everything together and make the actual API call. If you followed
along, your `src/main.ts` file should now look something like this:

```typescript
import {
  getCredentials,
  AtcServiceClient,
  GetVersionRequest,
  GetVersionResponse,
  ServiceError,
} from "auto-traffic-control";

function main() {
  const atcService = new AtcServiceClient("localhost:4747", getCredentials());

  atcService.getVersion(
    new GetVersionRequest(),
    (err: ServiceError | null, response: GetVersionResponse) => {
      if (err != null) {
        throw err;
      }

      const version = response.getVersion();

      if (version != null) {
        let versionString = [
          version.getMajor(),
          version.getMinor(),
          version.getPatch(),
        ].join(".");

        if (version.getPre() !== "") {
          versionString = versionString.concat(version.getPre());
        }

        console.log(
          `Auto Traffic Control is running version '${versionString}'`
        );
      } else {
        throw new Error("Requesting the version returned an empty response.");
      }
    }
  );
}

main();
```

You can run the file with `npm start`, but you will still get a connection error
because we haven't started the game.

Open the **itch** app, go to your library, and open **Auto Traffic Control**.
Hit the `Launch` button in the bottom right, and wait for the game to start.

![screenshot of the itch page for Auto Traffic Control](/img/tutorial/itch-atc-launch.png)

With the game open, run `npm start`. The program will connect to the game
server, request the game's version, and the print it to the terminal.

![screenshot of a successful API request](/img/tutorial/get-version.png)

:::info
If you get a connection error while the game is running, try disabling the
[sandbox](https://itch.io/docs/itch/using/sandbox.html) in **itch's**
[preferences](https://itch.io/docs/itch/using/preferences.html). It might be
blocking the incoming connections. Or download the game from [itch.io] and run
it as a standalone binary.
:::

Congrats! 🥳 You sent your first API request to the game.

Now that you know how to use
the [SDK](https://www.npmjs.com/package/auto-traffic-control)
and its [documentation](/docs/api), you are all set to start working on an
actual bot. Check out the [`GameService`] and
try to start the game. Hint: The [`GameService`] has a `startGame` method. 😉

Don't worry if you get stuck. We'll go over it in the next chapter together.

[`atcservice`]: /docs/api/Services/atc-service
[`gameservice`]: /docs/api/Services/game-service
[grpc]: https://grpc.io/
[itch.io]: https://jdno.itch.io/auto-traffic-control
[npmjs.com]: https://www.npmjs.com/package/auto-traffic-control
[release notes]: https://github.com/jdno/auto-traffic-control/releases
[schema definitions]: https://github.com/jdno/auto-traffic-control/tree/main/api
