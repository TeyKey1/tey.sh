<script lang="ts">
  import ShowMore from "$lib/ShowMore.svelte";

  export let title: string;
  export let tech: string;
</script>

<div class="p-4 my-4 border-dark border-2 rounded-lg">
  <h2 class="mt-0">{title}</h2>
  <p><em>{tech}</em></p>
  <p><strong><slot name="desc" /></strong></p>

  <ShowMore>
    <slot name="more" />
  </ShowMore>
</div>
