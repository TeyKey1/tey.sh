<script lang="ts">
  import type { PageData } from "./$types";

  // Components
  import SvelteSeo from "svelte-seo";

  export let data: PageData;

  const title = "Today I learned... | tey.sh";
  const description =
    "Today I learned... Interesting topics about various things found in the world of a software developer";
  const keywords = "Thierry Kühni, Portfolio, Terminal, Developer";
</script>

<SvelteSeo
  {title}
  {description}
  {keywords}
  jsonLd={{
    "@context": "https://schema.org",
    "@type": "WebPage",
    name: title,
    description,
    keywords,
  }}
  openGraph={{ title, description, type: "website", site_name: "tey.sh" }}
  twitter={{
    card: "summary",
    title,
    description,
  }}
/>

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
