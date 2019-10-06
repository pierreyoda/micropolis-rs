const path = require("path");
const withCSS = require("@zeit/next-css");

module.exports = () => withCSS({
  exportTrailingSlash: true,
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
    },
    node: {
      fs: "empty", // fixes npm packages that depend on `fs` module
    },
  }),
});
