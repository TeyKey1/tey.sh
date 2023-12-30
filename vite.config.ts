import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import path from "path";
import npmPackage from "./package.json";

const mdContentDir = path.resolve(__dirname, "./til-content");

export default defineConfig({
  plugins: [sveltekit()],
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
  define: {
    APP_REPOSITORY: JSON.stringify(npmPackage.repository.url),
  },
});
