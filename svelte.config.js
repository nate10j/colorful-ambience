import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: undefined,
			precompress: false,
			strict: true
		}),
		paths: {
			base: "/colorful-ambience/"
		}
	},

	preprocess: [mdsvex({
		extensions: ['.md', '.svx']
	})],
	extensions: ['.svelte', '.md', '.svx']
};

export default config;
