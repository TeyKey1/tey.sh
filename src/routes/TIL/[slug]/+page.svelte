<script lang="ts">
  import type { PageData } from "./$types";
  import type { SvelteComponent } from "svelte";

  // Components
  import SvelteSeo from "svelte-seo";
  import Article from "$lib/Article.svelte";

  let {data}: {data: PageData} = $props()

  type C = $$Generic<typeof SvelteComponent<any, any, any>>;

  let component = $derived(data.component as unknown as C);
  let {title, description, date, author, keywords} = $derived(data.frontmatter);
  let simplifiedDate = $derived( new Date(date).toLocaleDateString("en-gb"));
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

<Article {author} date={simplifiedDate} {title} {description}>
  {#snippet content()}
  <svelte:component this={component} />
  {/snippet}
</Article>
