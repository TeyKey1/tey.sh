---
title: Preventing SvelteKit from doing code analysis during build
description:
  During build, SvelteKit will load all your `+page/layout(.server).js` files
  and execute them for code analysis and further optimizations. In some cases,
  this behavior can lead to issues.
date: 2025-11-25T12:01:00
author: Thierry Kühni
keywords: SvelteKit, Svelte, build, analysis, $app/environment, building
published: true
---

Usually, I don't dive this deep into SvelteKit functionality when I work with
it. So, before hitting this problem, I was not aware that SvelteKit actually
executes certain user-written code during the build step. The docs briefly touch
this subject with the following content:

> SvelteKit will load your `+page/layout(.server).js` files (and all files they
> import) for analysis during the build. Any code that should not be executed at
> this stage must check that `building` from `$app/environment` is `false`
>
> <cite><a href="https://svelte.dev/docs/kit/building-your-app#During-the-build">SvelteKit
> docs</a></cite>

This means that nearly all code written by the user might be executed by
SvelteKit's build step. In most cases, this does not lead to issues as most code
does not rely on any external services and is not executed in the top-level of a
script (eg. executed immediately on import as opposed to being triggered by an
event etc.). Problems usually arise if you are using code that relies on
external services such as databases or other clients, as well as environment
variables that are not defined at build time.

## Issues with top-level initialization of databases or similar clients

A usual and valid pattern to initialize a database connection or other client is
to do it at the top level of a script, which means that the initialization code
is run immediately and only once when the file is imported. This has the benefit
that the connection is only initialized once and can then be used across
multiple requests without the need to reinitialize it. An example of this
pattern can look like this:

```typescript
import { MyDbClient } from "db-lib";

const dbClient = new MyDbClient({
  host: "...",
});

export const client = dbClient;
```

This `client` can then be imported by any SvelteKit file requiring it and be
used across multiple requests without reinitialization. This particular code can
already lead to an error during build-time in case the `MyDbClient` class
already tries to connect to a database that does not exist/run yet. Of course,
it would make no sense to have the DB running during build-time just to avoid
this error. Instead, we can use SvelteKit's `building` variable as described in
the cited docs above:

```typescript
import { MyDbClient } from "db-lib";
import { building } from "$app/environment";

let dbClient;

if (!building) {
  // Client creation must be skipped during build time as the DB is not running
  dbClient = new MyDbClient({
    host: "...",
  });
}

export const client = dbClient;
```

With this code, the creation of the DB client is skipped during build and does
not cause any further issues.

## Handling environment variables not known at build-time

In my case, the client I was using did not connect on initialization but
validated the provided host value for correctness. In case you hard-code the
value of host or use SvelteKit's
[static environment variables](https://svelte.dev/docs/kit/$env-static-private)
to define it this would not lead to any issues during build. As I was optimizing
the application for running as a Docker container, the baking of configuration
environment variables into the build was not an option. I used SvelteKit's
[dynamic environment variables](https://svelte.dev/docs/kit/$env-dynamic-private)
instead, which are read at run-time.

As you might already guess, this leads to similar issues during build as the
client initialization would now error due to the environment variable being an
empty string instead of a valid host value. To circumvent this, you can either
define a placeholder host value for the dynamic environment variable, or skip it
using SvelteKit's `building` variable similarly to the example above.
