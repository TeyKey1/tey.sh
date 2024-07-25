<script lang="ts">
  import { page } from "$app/stores";

  let { name, href = "/" + name }: { name: string; href?: string } = $props();

  let isActive = $derived(
    $page.route.id === href ||
      (`${$page.url.pathname}/`.startsWith(href) && href != "/"),
  );
</script>

<a
  {href}
  class="font-bold text-xl text-white px-1 mr-1 transition-colors hover:bg-accent leading-relaxed"
  class:bg-accent={isActive}
>
  {name}
</a>
