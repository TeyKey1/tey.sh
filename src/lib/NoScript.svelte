<script lang="ts">
  // Markdown content
  import { html as home } from "$content/home.md";
  import { html as projects } from "$content/projects.md";
</script>

<noscript class="block">
  <div class="text-center max-w-2xl mx-auto">
    <p class="text-dark-accent font-bold text-2xl py-4">
      Whoops, looks like you can't run JavaScript.
    </p>

    <p class="text-white-dim text-sm">
      Don't worry, I got you covered. You'll also find all the relevant
      information of the more flashy and fancy JS content on this page down
      below in plain html and css :)
    </p>
  </div>

  <div class="prose dark:prose-invert my-8">
    <!-- content -->
    {@html home}
    {@html projects}
  </div>
</noscript>
