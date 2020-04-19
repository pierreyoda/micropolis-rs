import React, { useState } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, number } from "@storybook/addon-knobs";

import StoryWrapper from "./StoryWrapper";
import CheckboxInput from "../CheckboxInput";

storiesOf("Common/Input", module)
  .addDecorator(withKnobs)
  .add("Checkbox", () => {
    const [checked, setChecked] = useState(false);

    return (
      <StoryWrapper>
        <CheckboxInput
          label="Click me!"
          checked={checked}
          onToggle={c => setChecked(c)}
          size={`${number("Size", 16, { min: 5, max: 100, step: 1 })}px`}
        />
      </StoryWrapper>
    );
  });
