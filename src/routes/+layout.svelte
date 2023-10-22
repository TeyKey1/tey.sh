<script lang="ts">
  // CSS
  import "../app.css";
  import "xterm/css/xterm.css";

  // Components
  import Header from "$lib/Header.svelte";
  import { onMount } from "svelte";

  let contentWrapper: HTMLDivElement;

  onMount(() => {
    window.addEventListener("resize", setContentWrapperHeight);

    setContentWrapperHeight();

    console.info(`
   _                   _     
  | |_ ___ _   _   ___| |__  
  | __/ _ \\ | | | / __| '_ \\ 
  | ||  __/ |_| |_\\__ \\ | | |
   \\__\\___|\\__, (_)___/_| |_|
           |___/             
  
  Hi there, looks like either the site crashed horribly or you're wondering how this page actually works. In both cases I recommend to have a look at the source: https://github.com/TeyKey1/tey.sh
          `);
  });

  /**
   * Sets the height of the content wrapper to the remaining viewport height after the header
   *
   * This is done using js as there isn't really a sound way to achieve such full-page websites with css
   */
  function setContentWrapperHeight() {
    if (!contentWrapper) return;

    const topOffset = contentWrapper.offsetTop;
    const viewportHeight = window.innerHeight;

    contentWrapper.style.height = `${viewportHeight - topOffset}px`;
  }
</script>

<div class="px-4 lg:px-8 bg-black w-full h-full">
  <div class="h-12 mb-4">
    <Header />
  </div>

  <div bind:this={contentWrapper}>
    <slot />
  </div>
</div>
