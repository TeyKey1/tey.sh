<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { APP_MODE, AppMode } from "$lib/appMode";
  import type { Writable } from "svelte/store";

  // Components
  import NoScript from "$lib/NoScript.svelte";

  // Markdown content
  import { html as home } from "$content/home.md";
  import { html as projects } from "$content/projects.md";
  import { html as skills } from "$content/skills.md";

  const appMode = getContext<Writable<AppMode>>(APP_MODE);
  let htmlContentWrapper: HTMLDivElement;
  let terminalWrapper: HTMLDivElement;

  onMount(async () => {
    handleAppModeChange($appMode); // Init component to current appMode

    const init = (await import("src-rust")).default; // Client side only import due to xtermjs

    await init();
  });

  $: handleAppModeChange($appMode);

  function handleAppModeChange(appMode: AppMode) {
    if (!htmlContentWrapper) return;

    if (appMode === AppMode.Html) {
      htmlContentWrapper.style.display = "block";
      terminalWrapper.style.display = "none";
    } else {
      htmlContentWrapper.style.display = "none";
      terminalWrapper.style.display = "block";
    }
  }
</script>

<NoScript />

<div id="terminal" class="w-full h-full" bind:this={terminalWrapper} />

<div
  id="html-content"
  class="prose dark:prose-invert py-8"
  bind:this={htmlContentWrapper}
>
  {@html home}
  <h1>Skills</h1>
  {@html skills}
  <h1>Projects</h1>
  {@html projects}
</div>
