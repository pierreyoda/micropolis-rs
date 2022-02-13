import path from "path";
// import { promises as fs } from "fs";

// const loadingPrefix = "@vite-plugin-wasm-pack-custom@";

/**
 * @see https://github.com/vitejs/vite/discussions/2584
 * @param {readonly string[]} packages
 * @returns {import('vite').Plugin}
 */
const wasmPackPlugin = packages => ({
  name: "vite-plugin-wasm-pack-custom",
  enforce: "pre",
  config: config => ({
    ...config,
    // disable optimizations for the already optimized WASM packages
    optimizeDeps: {
      ...config.optimizeDeps,
      exclude: [...(config.optimizeDeps?.exclude ?? []), ...packages],
    },
    server: {
      ...config.server,
      fs: {
        ...config.server?.fs,
        allow: [...(config.server?.fs?.allow ?? []), ...packages],
      },
    },
  }),
  // TODO: make aliasing "micropolis_wasm" work
  // resolveId(id) {
  //   // enable shortened absolute imports
  //   for (const p of packages) {
  //     if (p.endsWith(id)) {
  //       return `${loadingPrefix}${id}`;
  //     }
  //   }
  //   return null;
  // },
  // async load(id) {
  //   if (!id.startsWith(loadingPrefix)) {
  //     return null;
  //   }
  //   const realId = id.substring(loadingPrefix.length);
  //   const correspondingPackage = packages.find(p => p.endsWith(realId));
  //   if (!correspondingPackage) {
  //     return null;
  //   }
  //   const filepath = path.join(correspondingPackage, "./pkg/", `${realId}.js`);
  //   return fs.readFile(filepath, { encoding: "utf-8" });
  // },
  transform(code, id) {
    if (!packages.some(p => id.includes(p))) {
      return null;
    }

    // Vite appends version strings as URL parameters at the end that we need to strip
    const [modulePath] = id.split("?", 1);

    // Rewrite the WASM file path to use `new URL`.
    //
    // See Vite docs for details on how it works:
    // https://vitejs.dev/guide/assets.html#new-url-url-import-meta-url
    const wasmFile = path.basename(modulePath).replace(".js", "_bg.wasm");
    let replaced = false;
    code = code.replace(/(?<prefix>input\s*=\s*)[^\n;]+;/u, (_, prefix) => {
      replaced = true;
      // DUCT TAPE ALARM! ${'import'} is necessary because Vite rewrites import.* in
      // plugins...
      return `${prefix}new URL('./${wasmFile}', ${"import"}.meta.url).href;`;
    });
    if (!replaced) {
      throw new Error(`Unable to replace wasm-pack-ed input file reference: ${id}`);
    }

    return { code };
  },
});

export default wasmPackPlugin;
