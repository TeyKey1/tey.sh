<script lang="ts">
  import { browser } from "$app/environment";
  import { APP_MODE, AppMode } from "$lib/appMode";
  import { getContext, onMount } from "svelte";
  import type { Writable } from "svelte/store";

  const TYPEWRITER_INTERVAL_MS = 150;

  let appMode = getContext<Writable<AppMode>>(APP_MODE);
  let appTitle = "tey.html";

  let typewriterInterval: NodeJS.Timer | undefined;
  let typewriterRunning = false;

  onMount(() => {
    handleAppModeChange($appMode); // Init component to current appMode
  });

  $: if (browser) handleAppModeChange($appMode); // Needs a browser check to ensure that the prerendered html appTitle stays "tey.html"

  function handleAppModeChange(appMode: AppMode) {
    if (appMode === AppMode.Html) {
      appTitle = "tey.html";
    } else {
      appTitle = "tey.sh";
    }
  }

  function changeAppMode() {
    if ($appMode === AppMode.Html) {
      $appMode = AppMode.Terminal;
    } else {
      $appMode = AppMode.Html;
    }
  }

  function startTypewriterEffect() {
    if (typewriterRunning) return;

    typewriterRunning = true;

    const newFileEnding = $appMode === AppMode.Html ? "sh" : "html";

    let typewriterDelete = true;
    let typewriterCharIdx = 0;

    typewriterInterval = setInterval(() => {
      if (typewriterDelete) {
        const lastCharIdx = appTitle.length - 1;

        if (appTitle.charAt(lastCharIdx) === ".") {
          typewriterDelete = false;
        } else {
          appTitle = appTitle.slice(0, lastCharIdx);
        }
      } else {
        if (typewriterCharIdx >= newFileEnding.length) {
          clearInterval(typewriterInterval!); // Exit
        } else {
          appTitle += newFileEnding.charAt(typewriterCharIdx);
          typewriterCharIdx++;
        }
      }
    }, TYPEWRITER_INTERVAL_MS);
  }

  function endTypewriterEffect() {
    if (typewriterInterval) clearInterval(typewriterInterval);

    handleAppModeChange($appMode);

    typewriterRunning = false;
  }
</script>

<button
  id="app-mode"
  class="text-white font-bold text-xl select-none min-w-[8ch] text-left p-1 pr-2"
  on:click={() => {
    changeAppMode();
    endTypewriterEffect();
  }}
  on:mouseover={startTypewriterEffect}
  on:mouseleave={() => {
    endTypewriterEffect();
  }}
  on:focus={startTypewriterEffect}
  on:focusout={endTypewriterEffect}
>
  <!-- span creates a text-cursor-like border when the typewriter is running -->
  <span
    class:border-r-2={typewriterRunning}
    class:border-white={typewriterRunning}
    class:pr-[2px]={typewriterRunning}>{appTitle}</span
  >
</button>
