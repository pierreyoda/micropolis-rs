import { useState } from "react";
import { Meta } from "@storybook/react";
import { text } from "@storybook/addon-knobs";

import TextInput from "../TextInput";

export default {
  title: "Common/TextInput",
  component: TextInput,
} as Meta;

export const Regular = () => {
  const [value, setValue] = useState("");

  return (
    <TextInput
      value={value}
      onChange={setValue}
      placeholder={text("Placeholder", "Placeholder")}
    />
  );
};
