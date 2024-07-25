<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    selectedItemSlot,
    label,
    items,
  }: { selectedItemSlot: Snippet<[string]>; label: string; items: Array<string> } =
    $props();

  let selectedItem: string = $state(items[0] ?? "");
</script>

<div class="my-4">
  <div
    role="tablist"
    aria-label={label}
    class="flex justify-center flex-col sm:flex-row"
  >
    {#each items as item}
      <button
        role="tab"
        aria-selected={selectedItem === item}
        aria-controls={`tabpanel-${item}`}
        class="m-0.5 font-bold text-lg text-white px-4 transition-colors hover:bg-accent leading-relaxed"
        class:bg-accent={selectedItem === item}
        onclick={() => {
          selectedItem = item;
        }}
      >
        {item}
      </button>
    {/each}
  </div>

  <div
    role="tabpanel"
    id={`tabpanel-${selectedItem}`}
    class=" border-accent border-l-0"
  >
    {@render selectedItemSlot(selectedItem)}
  </div>
</div>
