import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		paths: {
			base: process.argv.includes('dev') ? '' : process.env.BASE_PATH
		},
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: "404.html",
			precompress: false,
			strict: true
		})
	},

	preprocess: [mdsvex({
		extensions: ['.md', '.svx']
	})],
	extensions: ['.svelte', '.md', '.svx']
};

export default config;
