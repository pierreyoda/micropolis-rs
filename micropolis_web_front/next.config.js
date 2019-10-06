const path = require("path");

module.exports = () => ({
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
  }),
});
