<script lang="ts">
  import { page } from "$app/stores";

  export let name: string;
  export let href = "/" + name;

  $: isActive =
    $page.route.id === href ||
    (`${$page.url.pathname}/`.startsWith(href) && href != "/");
</script>

<a
  {href}
  class="font-bold text-xl text-white px-1 mr-1 transition-colors hover:bg-dark leading-relaxed"
  class:bg-dark={isActive}
>
  {name}
</a>
