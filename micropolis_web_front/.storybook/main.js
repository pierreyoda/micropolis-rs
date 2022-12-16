const path = require("path");

/**
 * @see https://storybook.js.org/docs/react/configure/overview
 * @see https://github.com/storybookjs/storybook/blob/next/MIGRATION.md#70-breaking-changes
 */
module.exports = {
  core: {
    builder: "webpack5",
  },
  features: {
    postcss: true,
    storyStoreV7: true,
  },
  addons: [
    {
      name: "@storybook/addon-postcss",
      options: {
        postcssLoaderOptions: {
          implementation: require("postcss"),
        },
      },
    },
    "@storybook/addon-essentials",
    "@storybook/addon-a11y",
    "@storybook/addon-docs",
    "@storybook/addon-controls",
  ],
  typescript: {
    check: false,
    checkOptions: {},
  },
  stories: ["../components/**/**/*.stories.tsx"],
  webpackFinal: async config => ({
    ...config,
    module: {
      ...config.module,
      rules: [
        ...(config.module.rules || []),
        {
          test: /\.s[ca]ss$/,
          use: ["postcss-loader"],
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
