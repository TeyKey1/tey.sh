<script lang="ts">
  // CSS
  import "../app.css";
  import "xterm/css/xterm.css";
  import { onMount } from "svelte";

  // Components
  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";

  onMount(() => {
    console.info(`
   _                   _     
  | |_ ___ _   _   ___| |__  
  | __/ _ \\ | | | / __| '_ \\ 
  | ||  __/ |_| |_\\__ \\ | | |
   \\__\\___|\\__, (_)___/_| |_|
           |___/             
  
  Hi there, looks like either the site crashed horribly or you're wondering how this page actually works. In both cases I recommend to have a look at the source: ${APP_REPOSITORY}
          `);
  });
</script>

<div class="flex flex-col px-4 lg:px-8 max-w-5xl mx-auto min-h-full">
  <Header />

  <div class="flex-1 prose dark:prose-invert py-8 max-w-full">
    <slot />
  </div>

  <Footer />
</div>
