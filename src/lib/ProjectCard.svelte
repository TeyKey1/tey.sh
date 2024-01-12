<script lang="ts">
  import ShowMore from "$lib/ShowMore.svelte";

  export let title: string;
  export let tech: string;
</script>

<div class="px-4 my-8 border-dark border-l-4">
  <h2 class="mt-0">{title}</h2>
  <p><em>{tech}</em></p>
  <p><strong><slot name="desc" /></strong></p>

  <ShowMore>
    <slot name="more" />
  </ShowMore>
</div>
