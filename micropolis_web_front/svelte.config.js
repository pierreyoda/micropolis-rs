import fs from "fs-extra";
import path from "path";

const isString = value => typeof value === "string";

/**
 *   return a Vite plugin for handling wasm-pack crate
 *
 *   only use local crate
 *
 *   import wasmPack from 'vite-plugin-wasm-pack';
 *
 *   plugins: [wasmPack(['./my-local-crate'])]
 *
 *   only use npm crate, leave the first param to an empty array
 *
 *   plugins: [wasmPack([],['test-npm-crate'])]
 *
 *   use both local and npm crate
 *
 *   plugins: [wasmPack(['./my-local-crate'],['test-npm-crate'])]
 *
 * @param {string | string[]} crates local crates paths, if you only use crates from npm, leave an empty array here.
 */
function vitePluginWasmPack(crates) {
  const prefix = "@vite-plugin-wasm-pack@";
  const pkg = "pkg"; // default folder of wasm-pack module
  let config_base;
  let config_assetsDir;
  const cratePaths = isString(crates) ? [crates] : crates;
  // from ../../my-crate  ->  my_crate_bg.wasm
  const wasmFilename = cratePath => {
    return path.basename(cratePath).replace(/\-/g, "_") + "_bg.wasm";
  };
  // wasmfileName : CrateType
  const wasmMap = {};
  // 'my_crate_bg.wasm': {path:'../../my_crate/pkg/my_crate_bg.wasm', isNodeModule: false}
  cratePaths.forEach(cratePath => {
    const wasmFile = wasmFilename(cratePath);
    console.log(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>wasmFile", wasmFile);
    wasmMap[wasmFile] = {
      path: path.join(cratePath, pkg, wasmFile),
      isNodeModule: false,
    };
  });

  return {
    name: "vite-plugin-wasm-pack",
    enforce: "pre",
    configResolved(resolvedConfig) {
      config_base = resolvedConfig.base;
      config_assetsDir = resolvedConfig.build.assetsDir;
    },

    resolveId(id) {
      for (let i = 0; i < cratePaths.length; i++) {
        if (path.basename(cratePaths[i]) === id) return prefix + id;
      }
      return null;
    },

    async load(id) {
      if (id.indexOf(prefix) === 0) {
        id = id.replace(prefix, "");
        const modulejs = path.join("./node_modules", id, id.replace(/\-/g, "_") + ".js");
        const code = await fs.promises.readFile(modulejs, {
          encoding: "utf-8",
        });
        return code;
      }
    },

    async buildStart(_inputOptions) {
      const prepareBuild = async (cratePath, isNodeModule) => {
        const pkgPath = isNodeModule ? path.join("node_modules", cratePath) : path.join(cratePath, pkg);
        const crateName = path.basename(cratePath);
        if (!fs.existsSync(pkgPath)) {
          if (isNodeModule) {
            console.error("Error: " + `Can't find ${pkgPath}, run ${`npm install ${cratePath}`} first`);
          } else {
            console.error(
              "Error: " + `Can't find ${pkgPath}, run ${`wasm-pack build ${cratePath} --target web`} first`,
            );
          }
        }
        if (!isNodeModule) {
          // copy pkg generated by wasm-pack to node_modules
          try {
            await fs.copy(pkgPath, path.join("node_modules", crateName));
          } catch (error) {
            this.error(`copy crates failed`);
          }
        }
        // replace default load path with '/assets/xxx.wasm'
        const jsName = crateName.replace(/\-/g, "_") + ".js";
        console.log("jsName", jsName); // FIXME:
        const jsPath = path.join(cratePath, "./pkg/", jsName);
        const regex = /input = new URL\('(.+)'.+;/g;
        console.log("buildStart.jsPath", jsPath); // FIXME:
        let code = fs.readFileSync(path.resolve(jsPath), { encoding: "utf-8" });
        code = code.replace(regex, (_match, group1) => {
          return `input = "${path.posix.join(config_base, config_assetsDir, group1)}"`;
        });
        fs.writeFileSync(jsPath, code);
      };

      for await (const cratePath of cratePaths) {
        await prepareBuild(cratePath, false);
      }
    },

    configureServer({ middlewares }) {
      return () => {
        // send 'root/pkg/xxx.wasm' file to user
        middlewares.use((req, res, next) => {
          if (isString(req.url)) {
            const basename = path.basename(req.url);
            res.setHeader("Cache-Control", "no-cache, no-store, must-revalidate");
            console.log("middleware.basename", basename);
            const entry = wasmMap[basename];
            console.log("middleware.entry", entry);
            if (basename.endsWith(".wasm") && entry) {
              console.log("middleware.wasm<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
              res.writeHead(200, { "Content-Type": "application/wasm" });
              fs.createReadStream(entry.path).pipe(res);
            } else {
              next();
            }
          }
        });
      };
    },

    buildEnd() {
      // copy xxx.wasm files to /assets/xxx.wasm
      console.log("============buildEnd===========");
      wasmMap.forEach((crate, fileName) => {
        this.emitFile({
          type: "asset",
          fileName: `assets/${fileName}`,
          source: fs.readFileSync(crate.path),
        });
      });
    },
  };
}

import preprocess from "svelte-preprocess";
import { defineConfig } from "vite";
import { ViteRsw } from "vite-plugin-rsw";
// import vitePluginWasmPack from "vite-plugin-wasm-pack";
console.log("isorehrhioeshisreorsehioreshiosreihosreihorioseh");
import { fileURLToPath } from "url";
import { dirname, resolve } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
console.log(__dirname);

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
    // hydrate the <div id="svelte"> element in src/app.html
    target: "#svelte",
    vite: defineConfig({
      build: {
        minify: false,
      },
      plugins: [
        ViteRsw({
          root: "../",
          unwatch: ["*/pkg/*"],
          crates: [{ name: "micropolis_wasm", outDir: "./pkg/" }],
          profile: process.env.NODE_ENV === "production" ? "release" : "dev",
        }),
        // vitePluginWasmPack(resolve(__dirname, "../micropolis_wasm/")),
      ],
    }),
  },
};

export default config;
