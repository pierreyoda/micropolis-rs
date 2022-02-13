import { fileURLToPath } from "url";
import { defineConfig } from "vite";
import { dirname, resolve } from "path";
import preprocess from "svelte-preprocess";
import vitePluginWasmPackCustom from "./vite/wasm-plugin.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],

  kit: {
    vite: defineConfig({
      build: {
        minify: false,
      },
      plugins: [vitePluginWasmPackCustom([resolve(__dirname, "../micropolis_wasm/")])],
    }),
  },
};

export default config;
