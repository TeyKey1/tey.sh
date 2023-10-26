<svelte:options immutable />

<script lang="ts">
  export let message: string;
  export let hint: string | undefined = undefined;
  export let status: number | undefined = undefined;
</script>

<div class="text-center max-w-2xl mx-auto">
  {#if status}
    <p class="text-dark-accent font-bold text-4xl py-4">{status}</p>
  {/if}

  <p class="text-dark-accent font-bold text-2xl py-4">
    {message}
  </p>

  {#if hint}
    <p class="text-white-dim text-sm">
      {hint}
    </p>
  {/if}
</div>
