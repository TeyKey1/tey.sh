---
title: Using phantom data types in TypeScript (and Rust) APIs
description:
  Phantom data types (also called branded types) in TypeScript can be a powerful
  tool for creating ergonomic APIs, resulting in a better developer experience
  and long-term maintainability.
date: 2024-3-29T17:22:00
author: Thierry Kühni
keywords: typescript, phantom data, branded types, rust
published: true
---

Recently, I removed Google Analytics from a Website and used
[Umami](https://umami.is) analytics instead. Umami is a neat, open-source,
privacy-focused web analytics software you can easily self-host. Best of all, it
is GDPR compliant and does not use cookies (Begone pesky cookie consent banners
:D). Umami can track a multitude of
[custom events](https://umami.is/docs/track-events) happening on the website.
For example, I used a custom event to track clicks on social-media share buttons
to gain a better insight into which content is shared more frequently than
others.

The Umami function to report such a custom event from the browser looks like
this:

```typescript
track(event_name: string, event_data?: { [key: string]: string | number }): void;
```

With this function, we can assign an event name and append custom data to this
event. One could simply use the function as-is inside the existing codebase.
Still, given that I was confronted with a somewhat larger codebase where such
events need to be sent from multiple locations in multiple files, I opted for a
better approach.

Looking at Umami's `track` function, multiple things might go wrong in terms of
usage:

- Inconsistent event names being used for the same event (for example, "share"
  vs. "share click")
- Different `event_data` payloads being sent for the same event

...all of which would lead to inconsistent analytics data that would be harder
to manage and analyze.

An easy solution to the event name problem would be simply providing all
possible events as string constants and using those constants. However, this
does not prevent the event data from differing between different calls of the
track function for the same event, let alone prevent the developer from simply
using a custom string as an event name instead of the constants. Clearly, there
should be some kind of "invisible" connection between the event name and the
corresponding event data.

Coming from Rust, Rust's
[`PhantomData`](https://doc.rust-lang.org/stable/std/marker/struct.PhantomData.html)
type marker immediately sprang to mind. The wrapper function for Umami's `track`
would have looked like this:

```rust
use std::marker::PhantomData;

pub struct Event<'a, T> {
    pub name: &'a str,
    pub phantom: PhantomData<T>,
}

pub fn analytics_event<T>(event: &Event<T>, event_data: T) {
    track(event.name, event_data);
}

fn track<T>(_event_name: &str, _event_data: T) {}
```

As can be seen, the `Event` struct would contain the event name and the generic
`T`, which represents the type of `event_data` belonging to this specific event.
Thus, the `Event` struct is a phantom data type as T is not directly stored
within it (The `PhantomData` marker type is merely used so that the compiler
does not complain about the unused `T` generic and will get removed at
compile-time). As the `analytics_event` wrapper function definition now enforces
that both the provided `Event` has the generic `T` as well as the supplied
`event_data` is of type `T`, users of this function can now only provide the
matching `event_data` type for a specific `Event` struct:

```rust
use std::marker::PhantomData;

struct SharePayload {}
struct OtherPayload {}

// Create the share event and associate the SharePayload type to it
const SHARE_EVENT: Event<'static, SharePayload> = Event {
    name: "share",
    phantom: PhantomData,
};

fn main() {
    // This is OK
    analytics_event(&SHARE_EVENT, SharePayload {});

    // Big yikes! Mismatched types. Expected `SharePayload`, found `OtherPayload`
    analytics_event(&SHARE_EVENT, OtherPayload {});
}
```

A first naive way to translate this code into TypeScript looked like this:

```typescript
type Event<T> = string;

export function analyticsEvent<T>(event: Event<T>, payload: T) {
  track(event, payload);
}

// Possible analytic events
export const ANALYTICS_SHARE_EVENT: Event<{
  method: "link" | "twitter" | "facebook";
  url: string;
}> = "share";
```

This code looks reasonable. Again, we have the phantom type `Event` with the
generic `T` representing the associated payload type. The `analyticsEvent`
function again enforces with the generic `T` that the provided event and payload
types match. There are two problems with this, though. TypeScript rightly
complains that the generic `T` in the `Event` type is unused. Also, the
`ANALYTICS_SHARE_EVENT` constant ends up being coerced to a simple string
primitive, rendering all type checks useless.

After some googling, I found the right approach to using phantom data in
TypeScript. In TypeScript, the approach outlined below is also more popularly
called "type branding," in case you want to further read into the topic. The
modifications required to the `Event` type to enforce the phantom data without
TypeScript complaining look like this:

```typescript
declare const __phantom: unique symbol;
type Event<T> = string & { [__phantom]: T };
```

To create a rough equivalent to Rust's `PhantomData` type marker in TypeScript,
we use an intersection type where we add an object type to our base type (in our
case, the string storing the event name). This object contains a single computed
key `__phantom` of type `T` (The generic is used now). `__phantom` is a unique
symbol type, ensuring that each variant of the `Event` type has a unique
`__phantom` key. In our case, this is not necessarily important but is essential
if you want to create a general representation of a branded type in TypeScript.
The second advantage of using a computed property is that developers using
values with the `Event` type cannot see the `__phantom` key in their
auto-completion suggestions.

It is crucial to remember that the object containing the `__phantom` key will
never actually exist at runtime. It is simply a way to make the TypeScript
compiler aware of the phantom type, similar to Rust's dedicated `PhantomData`
type.

The approach to reaching the above code is described in more detail in
[this excellent blog post](https://egghead.io/blog/using-branded-types-in-typescript).

Concluding the resulting wrapper code around Umami's `track` function looks like
this:

```typescript
declare const __phantom: unique symbol;
type Event<T> = string & { [__phantom]: T };

export function analyticsEvent<T>(event: Event<T>, payload: T) {
	umami.track(event, payload);
}

// Possible analytic events
export const ANALYTICS_SHARE_EVENT = 'share' as Event<{
	method: 'link' | 'twitter' | 'facebook';
	url: string;
}>;
export const ANALYTICS_XYZ_EVENT = ...
```

If this code is located within its own TypeScript file (or provided as a
library), we now have a neat API that is hard to misuse. New Events can now only
be created within this file, ensuring that developers do not create the same
events twice. Additionally, the `analyticsEvent` function now properly ensures
that only matching payload data can be provided for the respective event.

Not only can we now ensure consistent analytics data reporting, but this code
will also be a lot easier to maintain in the long run (perhaps even by different
developers) as the phantom type and API clearly define what can and cannot be
done, thus catching many potential usage errors before they even hit the
runtime, even if the `analyticsEvent` function is used many times scattered
throughout the code base.

In Rust, I've grown quite fond of phantom data types, especially in libraries,
to prevent incorrect usage at compile time with zero runtime cost. For things
like database transactions (linking keys to the stored data type) or more
complex relationships between types in a library, it can be an essential tool
for enforcing correctness and creating a pleasant developer experience.
