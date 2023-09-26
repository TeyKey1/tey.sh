import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasmPack from 'vite-plugin-wasm-pack';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [wasmPack(["./src-rust"]), svelte()]
});
