<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<h1>Today I learned...</h1>

{#each data.posts as { slug, title, author, description, date }}
  <a href={`/TIL/${slug}`} class="no-underline">
    <div
      class="px-4 my-8 border-accent hover:border-white border-l-4 transition-colors"
    >
      <h2 class="mt-0">{title}</h2>
      <em>{date} {author}</em>
      <p class="mb-0" style="color: var(--tw-prose-invert-body)">
        {description}
      </p>
    </div>
  </a>
{/each}
