import React, { useState } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, text, boolean, number, select } from "@storybook/addon-knobs";

import Button, { buttonColors } from "../Button";
import StoryWrapper from "./StoryWrapper";

storiesOf("Common", module)
  .addDecorator(withKnobs)
  .add("Button", () => {
    const [pressedCount, setPressedCount] = useState(0);
    const width = number("Width", 250, { min: 25, max: 1500, step: 25 });
    const height = number("Height", 45, { min: 25, max: 150, step: 10 });

    return (
      <StoryWrapper>
        <span>Pressed {pressedCount} times</span>
        <Button
          disabled={boolean("Disabled?", false)}
          onToggle={() => setPressedCount(n => ++n)}
          width={`${width}px`}
          height={`${height}px`}
          color={select(
            "Color",
            buttonColors.reduce((acc, color) => ({ ...acc, [color]: color }), {}),
            buttonColors[0],
          )}
        >
          {text("Button Label", "Click me! 👍")}
        </Button>
      </StoryWrapper>
    );
  });
