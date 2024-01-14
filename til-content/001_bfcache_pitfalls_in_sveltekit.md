---
title: bfcache pitfalls in SvelteKit
description: In modern browsers, there's a thing called bfcache. If you never heard of it (like me a couple of weeks ago), this article might be worth a look.
date: 2024-1-14T15:52:00
author: Thierry Kühni
keywords: bfcache, SvelteKit, Svelte
published: true
---

Modern browsers happen to cache a lot of things. This has many benefits ranging from faster loading time to lesser data consumption, to name the most prominent ones. The most well-known browser cache is probably the HTTP cache, which stores responses to requests and serves any subsequent responses to requests from this cache (given it's allowed to do so).

The bfcache, which stands for back/forward cache, is a browser cache that aims to allow instant back and forward navigation of webpages. To make this possible, the browser stores a snapshot of the current page in memory once the user navigates to another page. This snapshot contains everything needed to fully restore the page, including the current frozen JavaScript execution state. In case the user hits the bfcache on a navigation, the browser fully restores the cached page without the need for any HTTP requests or initialization of JS code. The frozen JS code execution is simply resumed from that point on. This significantly improves user experience due to the page's near-instant loading time. For more specific information on how the bfcache works, I recommend the corresponding [web.dev article](https://web.dev/articles/bfcache).

## bfcache in SPAs

You might wonder if the bfcache also works for navigations within single-page applications. The short answer is that the bfcache only comes into play when the browser manages the navigation, which is not the case during SPA internal navigations using JavaScript. However, this is only part of the whole truth because you might still run into unexpected situations once the browser controls the navigation from a SPA. You can see further below in the SvelteKit example how this can lead to issues.

## Implications of bfcache hits

As web developers, we ideally want to ensure that our sites work correctly after being restored from bfcache. One might think that this should just work out of the box (which is true for a lot of cases) but there are some things to be aware of:

- The JS code execution is immediately stopped once the user navigates ([with some exceptions](https://web.dev/articles/bfcache#how_the_cache_works))
- Data displayed on the page might have become stale once the page is restored from bfcache
- Long-running connections such as WebSockets might need to be reestablished / properly closed

You can find further implications and best practices [here](https://web.dev/articles/bfcache#optimize_your_pages_for_bfcache).

## How bfcache and SvelteKit play together

With this in mind, we can now look at how this plays together with SvelteKit. For the [FliegerWeb](https://fliegerweb.com/de/home) webpage, I implemented an indefinite progress bar component, which would be displayed to the user in case a navigation would happen to take longer than 500ms.

A simplified version of this `ProgressBar.svelte` component would look like this:

```svelte
<script>
  import { beforeNavigate, afterNavigate } from "$app/navigation";

  let showProgressBar = false;

  beforeNavigate((nav) => {
    showProgressBar = true;
  });

  afterNavigate(() => {
    showProgressBar = false;
  });
</script>

{#if showProgressBar}
  <!-- Show the progress bar html... -->
{/if}
```

By the way, this code is for demonstration purposes only. Do not implement a navigation progress bar this way! Use SvelteKit's [navigating store](https://learn.svelte.dev/tutorial/navigating-store) instead, which already considers bfcache (see [SvelteKit#5613](https://github.com/sveltejs/kit/pull/5613)).

Back to the code: I used the `beforeNavigate` and `afterNavigate` hooks to either show or hide the progress bar on the page. This worked fine except for one case: When the user navigated to an external page and then used the back button to get back to FliegerWeb. The progress bar would then show up even though the navigation was finished and would only be hidden once the user completed another navigation within the site.

As you might have guessed, This behavior is caused due to hitting the bfcache. Let's break this down and see where the issue lies:

1. Navigation to an external page is intercepted by SvelteKit event listeners, which call the `beforeNavigate` hook -> Progress bar is shown.
2. Due to being a navigation to an external link, the browser takes over and stores a snapshot of Fliegerweb into bfcache
3. Once the user goes back, the browser loads the page from bfcache and resumes the JavaScript execution
4. Due to this, SvelteKit does not call `afterNavigate` as it cannot detect this navigation (The app is simply resumed), resulting in the progress bar not being hidden even though the navigation has already ended

So, in a nutshell, while `beforeNavigate` is being called, `afterNavigate` is never called when the browser restores the page from bfcache. As I already outlined above, if you need something that is aware of bfcache, use the [navigating store](https://learn.svelte.dev/tutorial/navigating-store), which works correctly even in case of the page getting restored from bfcache. Alternatively, in case you cannot use the navigating store, you can use the [pageshow browser event](https://web.dev/articles/bfcache#observe_when_a_page_is_restored_from_bfcache).

### Handling stale data

If your page contains data that might have become stale once it is restored from bfcache, you might need to add custom logic to load the new data. In SvelteKit, there is currently no built-in way to handle stale data after a bfcache hit. However, there is an open issue that discusses some possibilities for SvelteKit's load functions to become aware of bfcache and reload the data accordingly ([SvelteKit#9822](https://github.com/sveltejs/kit/issues/9822)).
