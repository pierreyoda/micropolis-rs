const path = require("path");
const webpack = require("webpack");
const withCSS = require("@zeit/next-css");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = () => withCSS({
  exportTrailingSlash: true,
  exportPathMap: () => ({
    "/": { page: "/" },
    "/about": { page: "/about" },
  }),
  webpack: config => ({
    ...config,
    plugins: [
      ...config.plugins,
      new CopyPlugin([
        {
          from: path.join(__dirname, "../img/"),
          to: path.join(__dirname, "./public/game/"),
        },
      ]),
    ],
    resolve: {
      ...config.resolve,
      alias: {
        ...config.resolve.alias,
        "@": path.resolve(__dirname, "./"),
      },
    },
    plugins: [
      ...config.plugins,
      new webpack.ProvidePlugin({
        PIXI: "pixi.js",
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "../micropolis_wasm/"),
        outDir: path.resolve(__dirname, "../micropolis_wasm/pkg/"),
      }),
    ],
    node: {
      fs: "empty", // fixes npm packages that depend on `fs` module
    },
  }),
});
