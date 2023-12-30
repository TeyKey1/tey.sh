<script lang="ts">
  export let date: string;
  export let author: string;
  export let title: string;
  export let description: string;
</script>

<article>
  <div class="mb-6">
    <h1 class="mb-2">{title}</h1>
    <em>{date} {author}</em>
  </div>
  <strong>{description}</strong>

  <slot />
</article>
