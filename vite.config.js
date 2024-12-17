import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [sveltekit(), wasmPack(["./noise-generator"])],

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
