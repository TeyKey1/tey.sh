<script lang="ts">
  import { onMount } from "svelte";

  const COPY_FEEDBACK_TIMEOUT_MS = 1000;

  export let code: string;

  let hoveringCode = false;
  let showCopyButton = false;
  let copySuccessful = false;
  let copyError = false;
  let copyButton: HTMLButtonElement;

  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(code);
    } catch (err) {
      console.error("Failed to copy code to clipboard: ", err);

      copyError = true;
      setTimeout(() => (copyError = false), COPY_FEEDBACK_TIMEOUT_MS);

      return;
    }

    copySuccessful = true;
    setTimeout(() => (copySuccessful = false), COPY_FEEDBACK_TIMEOUT_MS);
  }

  onMount(() => {
    const codeBlock = copyButton.parentElement!;

    codeBlock.addEventListener("mouseenter", () => {
      hoveringCode = true;
    });

    codeBlock.addEventListener("mouseleave", () => {
      hoveringCode = false;
    });
  });

  $: showCopyButton = hoveringCode || copySuccessful;
</script>

<button
  aria-label="copy code to clipboard"
  disabled={copySuccessful}
  on:click={copyToClipboard}
  class:hidden={!showCopyButton}
  class="absolute top-0 right-0 m-2 p-2 text-accent hover:text-white transition-colors"
  bind:this={copyButton}
>
  {#if copySuccessful}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-8 h-8 text-cli-green"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
      />
    </svg>
  {:else if copyError}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-8 h-8 text-dark-error"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
      />
    </svg>
  {:else}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-8 h-8"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M15.666 3.888A2.25 2.25 0 0 0 13.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 0 1-.75.75H9a.75.75 0 0 1-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 0 1-2.25 2.25H6.75A2.25 2.25 0 0 1 4.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 0 1 1.927-.184"
      />
    </svg>
  {/if}
</button>
