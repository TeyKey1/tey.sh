import CopyCodeButton from "./CopyCodeButton.svelte";

/**
 * Adds code copy buttons to all code blocks found on the current page.
 *
 * # Caution
 * Only call this function in a browser context
 */
export function addCodeCopyButtons() {
  const codeBlocks = document.querySelectorAll("pre code");

  for (const codeBlock of codeBlocks) {
    const code = codeBlock.textContent;
    const target = codeBlock.parentElement;

    if (code === null || target === null) {
      continue; // Do not display copy button
    }

    new CopyCodeButton({ target, props: { code } });
  }
}
