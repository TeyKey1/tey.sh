<script lang="ts">
  import { APP_MODE, AppMode } from "$lib/appMode";
  import { getContext, onMount } from "svelte";
  import type { Writable } from "svelte/store";

  // Components
  import GitHub from "$lib/logo-links/GitHub.svelte";
  import Matrix from "$lib/logo-links/Matrix.svelte";

  let titleElement: HTMLButtonElement;

  let appMode = getContext<Writable<AppMode>>(APP_MODE);

  onMount(() => {
    handleAppModeChange($appMode); // Init component to current appMode
  });

  $: handleAppModeChange($appMode);

  function handleAppModeChange(appMode: AppMode) {
    if (!titleElement) return;

    if (appMode === AppMode.Html) {
      titleElement.innerText = "tey.html";
    } else {
      titleElement.innerText = "tey.sh";
    }
  }

  function changeAppMode() {
    if ($appMode === AppMode.Html) {
      $appMode = AppMode.Terminal;
    } else {
      $appMode = AppMode.Html;
    }
  }
</script>

<div class="flex w-full h-full justify-between py-4">
  <button
    id="app-mode"
    class="text-white font-bold text-xl select-none"
    on:click={changeAppMode}
    bind:this={titleElement}
  >
    tey.html
  </button>
  <div class="flex">
    <GitHub />
    <Matrix />
  </div>
</div>
