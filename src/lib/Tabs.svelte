<script lang="ts">
  export let label: string;
  export let items: Array<string>;

  let selectedItem: string = items[0] ?? "";
</script>

<div class="my-4 border-dark border-2 rounded-lg">
  <div role="tablist" aria-label={label}>
    {#each items as item}
      <button
        role="tab"
        aria-selected={selectedItem === item}
        aria-controls={`tabpanel-${item}`}
        class="font-bold text-lg text-white px-4 transition-colors hover:bg-dark leading-relaxed"
        class:bg-dark={selectedItem === item}
        on:click={() => {
          selectedItem = item;
        }}
      >
        {item}
      </button>
    {/each}
  </div>

  <div role="tabpanel" id={`tabpanel-${selectedItem}`} class="px-4">
    <slot {selectedItem} />
  </div>
</div>
