<script lang="ts">
  import type { PageData } from "./$types";
  import type { SvelteComponent } from "svelte";

  // Components
  import SvelteSeo from "svelte-seo";
  import Article from "$lib/Article.svelte";

  export let data: PageData;

  type C = $$Generic<typeof SvelteComponent<any, any, any>>;
  $: component = data.component as unknown as C;

  $: ({ title, description, date, author, keywords } = data.frontmatter);
</script>

<SvelteSeo
  {title}
  {description}
  {keywords}
  jsonLd={{
    "@context": "https://schema.org",
    "@type": "Article",
    headline: title,
    description,
    author,
    datePublished: new Date(date).toISOString(),
    genre: "programming",
    keywords,
  }}
  openGraph={{ title, description, type: "website", site_name: "tey.sh" }}
  twitter={{
    card: "summary",
    title,
    description,
  }}
/>

<Article {author} {date} {title} {description}>
  <svelte:component this={component} />
</Article>
