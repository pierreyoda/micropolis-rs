import { configure, addDecorator } from "@storybook/react";

configure(require.context('../', true, /\.stories\.(tsx|mdx)$/), module);
