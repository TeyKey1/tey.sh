import { sveltekit } from "@sveltejs/kit/vite";
import wasmPack from "vite-plugin-wasm-pack";
import { plugin as markdown, Mode } from "vite-plugin-markdown";
import { defineConfig } from "vite";
import path from "path";

const mdContentDir = path.resolve(__dirname, "./content");

export default defineConfig({
  plugins: [
    wasmPack(["./src-rust"]),
    markdown({ mode: [Mode.HTML] }),
    sveltekit(),
  ],
  resolve: {
    alias: {
      $content: mdContentDir,
    },
  },
  server: {
    fs: {
      allow: [mdContentDir],
    },
  },
});
