import React from "react";
import createCache from "@emotion/cache";
import { CacheProvider } from "@emotion/react";
import { GlobalStyles } from "twin.macro";
import "@storybook/addon-console";

const cache = createCache({ prepend: true, key: "twin" });

export const parameters = {
  layout: "centered",
};

export const decorators = [
  Story => (
    <CacheProvider value={cache}>
      <GlobalStyles />
      <Story />
    </CacheProvider>
  ),
];
