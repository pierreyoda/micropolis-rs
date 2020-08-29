import { themes } from '@storybook/theming';

require("@/assets/styles/tailwind.css");

// or global addParameters
export const parameters = {
  docs: {
    theme: themes.dark,
  },
};
