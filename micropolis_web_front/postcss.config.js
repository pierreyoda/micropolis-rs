/* eslint-disable global-require */

const production = process.env.APP_ENV === "production";

module.exports = {
  plugins: [
    require("tailwindcss")("./tailwind.config.js"),
    production &&
      require("@fullhuman/postcss-purgecss")({
        content: ["pages/**/*.tsx", "layouts/**/*.tsx", "components/**/*.tsx"],
        extractors: [
          {
            extensions: ["html", "js", "ts", "css", "scss", "jsx", "tsx"],
            extractor: class TailwindExtractor {
              static extract(content) {
                return content.match(/[A-Za-z0-9-_:\\/]+/g) || [];
              }
            },
          },
        ],
        whitelist: ["html"],
      }),
    require("autoprefixer")({}),
    production &&
      require("cssnano")({
        preset: "default",
      }),
  ],
};
