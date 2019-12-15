const path = require("path");
const withCSS = require("@zeit/next-css");
const CopyPlugin = require("copy-webpack-plugin");

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
    node: {
      fs: "empty", // fixes npm packages that depend on `fs` module
    },
  }),
});
