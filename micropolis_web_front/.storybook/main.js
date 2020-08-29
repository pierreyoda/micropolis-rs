const path = require("path");

module.exports = {
  addons: [
    "@storybook/addon-essentials",
    "@storybook/addon-knobs",
    "@storybook/addon-a11y",
  ],
  stories: ["../components/**/**/*.stories.tsx"],
  webpackFinal: async config => ({
    ...config,
    resolve: {
      ...config.resolve,
      alias: {
        ...config.resolve.alias,
        "@": path.resolve(__dirname, "../"),
      },
    },
    node: {
      fs: "empty",
    },
  }),
};
