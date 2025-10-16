import { mdsvex } from "mdsvex";
import mdsvexConfig from "./mdsvex.config.js";
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: [".svelte", ...mdsvexConfig.extensions],
  preprocess: [vitePreprocess(), mdsvex(mdsvexConfig)],

  vitePlugin: {
    dynamicCompileOptions({ filename }) {
      if (filename.includes("node_modules")) return { runes: undefined }; // Do not opt-in for runes only mode on deps
    },
  },

  kit: {
    adapter: adapter(),
    paths: {
      relative: false,
    },
  },

  compilerOptions: {
    runes: true,
  },
};

export default config;
