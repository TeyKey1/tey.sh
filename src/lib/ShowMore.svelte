<script lang="ts">
  import { slide } from "svelte/transition";

  let showMore = false;
</script>

{#if showMore}
  <div transition:slide>
    <slot />
  </div>
{/if}

<button
  on:click={() => {
    showMore = !showMore;
  }}>{showMore ? "Show less" : "Read more..."}</button
>
