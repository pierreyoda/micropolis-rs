const path = require("path");

module.exports = ({ config }) => ({
  ...config,
  module: {
    ...config.module,
    rules: [
      ...config.module.rules, {
        test: /\.(ts|tsx)$/,
        loader: require.resolve("babel-loader"),
        options: {
          presets: [require.resolve('babel-preset-react-app')],
        },
      },
    ],
  },
  resolve: {
    ...config.resolve,
    alias: {
      ...config.resolve.alias,
      "@": path.resolve(__dirname, "../"),
    },
    extensions: [
      ...config.resolve.extensions,
      ".ts",
      ".tsx",
    ],
  },
  node: {
    fs: "empty",
  },
});
