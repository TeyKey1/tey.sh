<script lang="ts">
  import { onMount } from "svelte";
  import { addCodeCopyButtons } from "$lib/code/copyCodeButton";

  export let date: string;
  export let author: string;
  export let title: string;
  export let description: string;

  onMount(() => {
    addCodeCopyButtons();
  });
</script>

<article>
  <div class="mb-6">
    <h1 class="mb-2">{title}</h1>
    <em>{date} {author}</em>
  </div>
  <strong>{description}</strong>

  <slot />
</article>
