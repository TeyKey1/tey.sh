<script lang="ts">
  import type { Snippet } from "svelte";
  import ShowMore from "$lib/ShowMore.svelte";

  let {
    desc,
    more,
    title,
    tech,
  }: { desc: Snippet; more: Snippet, title: string; tech: string } = $props();
</script>

<div class="px-4 my-8 border-accent border-l-4">
  <h2 class="mt-0">{title}</h2>
  <p><em>{tech}</em></p>
  <p><strong>{@render desc()}</strong></p>

  <ShowMore>
    {@render more()}
  </ShowMore>
</div>
