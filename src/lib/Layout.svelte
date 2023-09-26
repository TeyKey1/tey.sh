<script lang="ts">
    // CSS
    import "../app.css";
    import "xterm/css/xterm.css";

    // Components
    import Header from "./Header.svelte";
    import { onMount } from "svelte";

    let contentWrapper: HTMLDivElement;

    onMount(() => {
        window.addEventListener("resize", setContentWrapperHeight);

        setContentWrapperHeight();
    });

    /**
     * Sets the height of the content wrapper to the remaining viewport height after the header
     *
     * This is done using js as there isn't really a sound way to achieve such full-page websites with css
     */
    function setContentWrapperHeight() {
        if (!contentWrapper) return;

        const topOffset = contentWrapper.offsetTop;
        const viewportHeight = window.innerHeight;

        contentWrapper.style.height = `${viewportHeight - topOffset}px`;
    }
</script>

<div class="px-4 lg:px-8 bg-black w-full h-full">
    <div class="h-12 mb-4">
        <Header />
    </div>

    <div bind:this={contentWrapper}>
        <slot />
    </div>
</div>
