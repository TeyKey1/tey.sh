<script lang="ts">
  import { onMount, type Snippet } from "svelte";
  import { addCodeCopyButtons } from "$lib/code/copyCodeButton";

  let {
    date,
    author,
    title,
    description,
    content,
  }: {
    date: string;
    author: string;
    title: string;
    description: string;
    content: Snippet;
  } = $props();

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

  {@render content()}
</article>
