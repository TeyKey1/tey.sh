<script lang="ts">
    import type { Snippet } from "svelte";
  import { slide } from "svelte/transition";

  let {children}: {children: Snippet} = $props();

  let showMore = $state(false);
</script>

{#if showMore}
  <div transition:slide>
    {@render children()}
  </div>
{/if}

<button
  onclick={() => {
    showMore = !showMore;
  }}>{showMore ? "Show less" : "Read more..."}</button
>
