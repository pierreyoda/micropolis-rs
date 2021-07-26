const path = require("path");

module.exports = {
  core: {
    builder: "webpack5",
  },
  addons: [
    "@storybook/addon-essentials",
    "@storybook/addon-knobs",
    "@storybook/addon-a11y",
    "@storybook/addon-docs",
    "@storybook/addon-controls",
  ],
  stories: ["../components/**/**/*.stories.tsx"],
  webpackFinal: async config => ({
    ...config,
    module: {
      ...config.module,
      rules: [
        ...(config.module.rules || []),
        {
          test: /\.s[ca]ss$/,
          use: [
            "style-loader",
            "css-loader",
            "postcss-loader",
            "sass-loader",
          ],
        },
      ],
    },
    resolve: {
      ...config.resolve,
      alias: {
        ...config.resolve.alias,
        "@": path.resolve(__dirname, "../"),
      },
      fallback: {
        ...config.resolve.fallback,
        fs: false, // fixes npm packages that depend on `fs` module
      },
    },
  }),
};
