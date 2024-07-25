<script lang="ts">
  let {
    message,
    hint,
    status,
  }: { message: string; hint?: string; status?: number } = $props();
</script>

<div class="text-center max-w-2xl mx-auto">
  {#if status}
    <p class="text-dark-error font-bold text-4xl py-4">{status}</p>
  {/if}

  <p class="text-dark-error font-bold text-2xl py-4">
    {message}
  </p>

  {#if hint}
    <p class="text-white-dim text-sm">
      {hint}
    </p>
  {/if}
</div>
