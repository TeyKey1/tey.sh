<script lang="ts">
  // CSS
  import "../app.css";
  import "xterm/css/xterm.css";
  import { writable } from "svelte/store";
  import { APP_MODE, AppMode } from "$lib/appMode";
  import { onMount, setContext } from "svelte";

  // Components
  import Header from "$lib/Header.svelte";

  const appMode = writable<AppMode>(AppMode.Terminal);
  let contentWrapper: HTMLDivElement;

  setContext(APP_MODE, appMode);

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
   *
   * Required because xtermjs will not fill the entire height of the page if wrapper is not sized
   */
  function setContentWrapperHeight() {
    if (!contentWrapper) return;

    const topOffset = contentWrapper.offsetTop;
    const viewportHeight = window.innerHeight;

    contentWrapper.style.height = `${viewportHeight - topOffset}px`;
  }
</script>

<svelte:head>
  <script>
    // If this script gets executed the user can run JS
    // In this case we hide the html-content once it gets attached to the dom and switch the appMode to terminal mode
    //
    // This avoids flashing of content unrelated to the current mode but still ensures that the site correctly displays in html mode
    // if the user cannot run JS
    let foundHtmlWrapper = false;
    let foundModeButton = false;

    const observer = new MutationObserver((mutationList, observer) => {
      for (const mutation of mutationList) {
        for (const node of mutation.addedNodes) {
          if (node.nodeName !== "DIV" && node.nodeName !== "BUTTON") break;

          if (node.id === "html-content") {
            node.style.display = "none";
            foundHtmlWrapper = true;
          } else if (node.id === "app-mode") {
            // Change button from default "tey.html" to "tey.sh" as the appMode starts in terminal mode if the user can run JS
            node.innerText = "tey.sh";
            foundModeButton = true;
          }

          if (foundHtmlWrapper && foundModeButton) {
            observer.disconnect();
            console.log("disconnect");
            return;
          }
        }
      }
    });

    observer.observe(document, {
      childList: true,
      subtree: true,
    });
  </script>
</svelte:head>

<div class="px-4 lg:px-8 w-full">
  <div class="h-12 mb-4">
    <Header />
  </div>

  <div bind:this={contentWrapper}>
    <slot />
  </div>
</div>
