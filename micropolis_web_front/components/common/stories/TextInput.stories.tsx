import React, { useState } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, text } from "@storybook/addon-knobs";

import TextInput from "../TextInput";
import StoryWrapper from "./StoryWrapper";

storiesOf("Common/Input", module)
  .addDecorator(withKnobs)
  .add("TextInput", () => {
    const [value, setValue] = useState("");

    return (
      <StoryWrapper>
        <TextInput
          value={value}
          onChange={setValue}
          placeholder={text("Placeholder", "Placeholder")}
        />
      </StoryWrapper>
    );
  });
