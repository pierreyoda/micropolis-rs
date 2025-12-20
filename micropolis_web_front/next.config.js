const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = () => ({
  trailingSlash: true,
  exportPathMap: () => ({
    "/": { page: "/" },
    "/about": { page: "/about" },
  }),
  webpack: config => ({
    ...config,
    resolve: {
      ...config.resolve,
      alias: {
        ...config.resolve.alias,
        "@": path.resolve(__dirname, "./"),
      },
      fallback: {
        ...config.resolve.fallback,
        fs: false, // fixes npm packages that depend on `fs` module
      },
    },
    experiments: {
      ...config.experiments,
      syncWebAssembly: true,
      // asyncWebAssembly: true,
    },
    plugins: [
      ...config.plugins,
      new CopyPlugin({
        patterns: [
          {
            from: path.join(__dirname, "../img/"),
            to: path.join(__dirname, "./public/game/"),
          },
          {
            from: path.join(__dirname, "../micropolis_utils/output/"),
            to: path.join(__dirname, "./public/utils-output/"),
          },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "../micropolis_wasm/"),
        outDir: path.resolve(__dirname, "./pkg"),
        // forceMode: "development",
        watchDirectories: [
          path.resolve(__dirname, "../micropolis_core"),
          path.resolve(__dirname, "../micropolis_wasm"),
        ],
        forceWatch: true,
        extraArgs: "--target web",
      }),
    ],
  }),
});
