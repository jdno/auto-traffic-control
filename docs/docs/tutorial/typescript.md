---
sidebar_position: 3
---

# Setting up TypeScript

In this tutorial, we will use [TypeScript] to implement our bot.

:::caution
The tutorial assumes that you already have [Node.js](https://nodejs.org/) and
[npm](https://npmjs.com) installed on your machine.
:::

If you want to use a different programming language, check out the
[Quickstart](/docs#download-the-client-library) guide for a list of officially
supported programming languages. If you're missing your language of choice, you
can probably generate API bindings from the game's [Protocol Buffers][protobufs]
yourself. However, this is not covered by this tutorial.

## Setting up a new project

First, let's start by setting up a new project.

Create a new directory for the project at a location of your choosing. Then
open a terminal, `cd` into the new directory, and run the following command to
get started:

```shell
npm init -y
```

Open the newly created `package.json` file and review its contents. Feel free to
give your bot a funky name, and change anything else that doesn't suit your
needs.

:::tip
We recommend that you use a version control system like [Git] for your bot so
that you can try out different ideas easily.
:::

## Adding TypeScript

We'll follow the [instructions](https://www.typescriptlang.org/download) of
[TypeScript] to add it to the project, and add it as a dependency to our
project:

```shell
npm install --save-dev typescript
```

We also want to add [`ts-node`] as a dependency. With [`ts-node`], we can run
the bot without having to first compile the code to JavaScript. You'll see what
that means in a second. First, add it as a dependency:

```shell
npm install --save-dev ts-node
```

## Creating the script

For now, we'll create a simple script that we can run and that'll
print `Hello, World!` to the command-line. It's enough to prove
that [TypeScript] is working,
without overwhelming us at this point with other challenges.

Inside the project folder, create a new directory called `src`. In there, create
a file called `main.ts`. At this point, your directory structure should look
like this:

```text
.
├── node_modules
│   └── ...
├── package-lock.json
├── package.json
└── src
    └── main.ts
```

Open `src/main.ts`, and add the following content:

```typescript
function main() {
  console.log("Hello, World!");
}

main();
```

That's it for now. Let's run the script.

## Running the script

Thanks to [`ts-node`], we can run the script without having to compile it to
JavaScript first.

```shell
$ npx ts-node src/main.ts
Hello, World!
```

We can make it a bit easier to run the script, though. Let's open up
`package.json`, and make this command our _start_ script. Find the `"scripts"`
section in the file, and add the following `"start"` command:

```json
{
  "scripts": {
    "start": "npx ts-node src/main.ts"
  }
}
```

We can now run our bot by simply running `npm start`. That's a lot easier to
remember.

```shell
$ npm start

> node-traffic-controller@0.0.0 start
> npx ts-node src/main.ts

Hello, World!
```

This concludes setting up [TypeScript] in the project. We have a script that we
can run, and the next step is adding the game's Node SDK and making an API
request.

[git]: https://git-scm.com/
[grpc]: https://grpc.io
[protobufs]: https://github.com/jdno/auto-traffic-control/tree/main/api
[`ts-node`]: https://github.com/TypeStrong/ts-node
[typescript]: https://www.typescriptlang.org/
