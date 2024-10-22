<script lang="ts">
    import type { Snippet } from "svelte";

    let {children}: {children: Snippet} = $props();
</script>

<div class="InfoBox border-cli-yellow">
    <p class="flex items-center text-cli-yellow font-extrabold">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 mr-2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z" />
        </svg>
        Warning
    </p>
    
    <em>{@render children()}</em>
</div>

<style>
    .InfoBox {
        border-inline-start-width: 0.25rem;
        margin-top: 1.6em;
        margin-bottom: 1.6em;
        padding-inline-start: 1em;
    }
</style>