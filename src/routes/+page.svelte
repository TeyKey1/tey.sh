<script lang="ts">
  import { onMount } from "svelte";

  // Components
  import NoScript from "$lib/NoScript.svelte";

  onMount(async () => {
    const init = (await import("src-rust")).default; // Client side only import due to xtermjs

    await init();
  });
</script>

<div id="terminal" class="w-full h-full" />

<NoScript />
