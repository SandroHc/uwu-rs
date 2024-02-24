import {readFileSync} from "fs";
import {fileURLToPath} from "url";
import {sveltekit} from "@sveltejs/kit/vite";
import {defineConfig} from "vitest/config";

export default defineConfig({
	build: {
		target: "esnext",
	},
	define: {
		__UWU_VERSION__: JSON.stringify(getUwuVersion()),
	},
	plugins: [
		sveltekit(),
	],
	server: {
		fs: {
			allow: [
				"../crates/uwu_wasm/pkg/uwu_bg.wasm",
			]
		}
	},
	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"]
	}
});

function getUwuVersion() {
	const file = fileURLToPath(new URL("package.json", import.meta.url));
	const json = readFileSync(file, "utf8");
	const pkg = JSON.parse(json);
	return pkg["dependencies"]["uwu-rs"]
}
