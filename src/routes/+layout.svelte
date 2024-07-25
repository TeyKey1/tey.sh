<script lang="ts">
  import { onMount, type Snippet } from "svelte";

  // CSS
  import "../css/app.css";
  import "../css/prism.css";

  // Components
  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";

  let { children }: { children: Snippet } = $props();

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
    {@render children()}
  </div>

  <Footer />
</div>
