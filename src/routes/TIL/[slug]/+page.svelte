<script lang="ts">
  import Article from "$lib/Article.svelte";

  import type { PageData } from "./$types";
  import type { SvelteComponentTyped } from "svelte/internal";

  export let data: PageData;

  type C = $$Generic<typeof SvelteComponentTyped<any, any, any>>;
  $: component = data.component as unknown as C;
</script>

<Article
  author={data.frontmatter.author}
  date={data.frontmatter.date}
  title={data.frontmatter.title}
  description={data.frontmatter.description}
>
  <svelte:component this={component} />
</Article>
