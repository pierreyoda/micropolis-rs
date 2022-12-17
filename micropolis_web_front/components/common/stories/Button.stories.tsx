import React from "react";
import { Meta } from "@storybook/react";

import Button from "../Button";

export const Primary = () => (
  <Button
    onToggle={() => {
      console.log("Clicked!");
    }}
  >
    Click me! 👍
  </Button>
);

export default {
  title: "Common/Button",
  component: Button,
} as Meta;
