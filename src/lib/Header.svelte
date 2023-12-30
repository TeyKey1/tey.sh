<script lang="ts">
  // Components
  import GitHub from "$lib/logo-links/GitHub.svelte";
  import Matrix from "$lib/logo-links/Matrix.svelte";
  import Menu from "$lib/Menu/Menu.svelte";
</script>

<div class="flex w-full justify-between py-4">
  <Menu />
  <div class="flex">
    <GitHub />
    <Matrix />
  </div>
</div>
