<script lang="ts">
    import init from "src-rust";
    import { onMount } from "svelte";

    // Components
    import Layout from "./lib/Layout.svelte";

    onMount(async () => {
        await init();
    });
</script>

<Layout>
    <div id="terminal" class="w-full h-full" />

    <div
        role="button"
        class="absolute bottom-1/2 left-0 font-mono text-white text-xs -rotate-180 pr-1"
        style="writing-mode: vertical-lr; text-orientation: mixed;"
    >
        Help, I hate CLI's :[
    </div>
</Layout>
