<script lang="ts">
  export let label: string;
  export let items: Array<string>;

  let selectedItem: string = items[0] ?? "";
</script>

<div class="my-4">
  <div role="tablist" aria-label={label} class="flex justify-center">
    {#each items as item}
      <button
        role="tab"
        aria-selected={selectedItem === item}
        aria-controls={`tabpanel-${item}`}
        class="font-bold text-lg text-white px-4 transition-colors hover:bg-accent leading-relaxed"
        class:bg-accent={selectedItem === item}
        on:click={() => {
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
    <slot {selectedItem} />
  </div>
</div>
