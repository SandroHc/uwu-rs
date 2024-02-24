import adapter from "@sveltejs/adapter-static";
import {vitePreprocess} from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			precompress: true,
		}),
		alias: {
			$wasm: "../crates/uwu_wasm/pkg",
		},
		inlineStyleThreshold: Infinity,
	},
};

export default config;
