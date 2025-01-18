import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	base: "colorful-ambience",
	plugins: [sveltekit(), wasmPack(['./noise_generator'])], 

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
}
});
