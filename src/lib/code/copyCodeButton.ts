import { mount } from "svelte";
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

    if (code === null || target === null || target.parentElement === null) {
      continue; // Do not display copy button
    }

    // Wrap the code element in a div to prevent the button being scrolled with the content
    const wrapperDiv = document.createElement("div");
    wrapperDiv.classList.add("relative");

    target.parentElement.insertBefore(wrapperDiv, target);
    wrapperDiv.appendChild(target);

    mount(CopyCodeButton, { target: wrapperDiv, props: { code } });
  }
}
