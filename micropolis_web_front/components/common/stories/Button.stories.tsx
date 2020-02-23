import React, { useState } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, text, boolean, number } from "@storybook/addon-knobs";

import Button from "../Button";
import StoryWrapper from "./StoryWrapper";

storiesOf("Common", module)
  .addDecorator(withKnobs)
  .add("Button", () => {
    const [toggled, setToggled] = useState(false);
    const width = number("Width", 250, { min: 25, max: 1500, step: 25 });
    const height = number("Height", 45, { min: 25, max: 150, step: 10 });

    return (
      <StoryWrapper>
        <span>{toggled ? "active" : "inactive"}</span>
        <Button
          disabled={boolean("Disabled?", false)}
          active={toggled}
          onToggle={() => setToggled(t => !t)}
          width={`${width}px`}
          height={`${height}px`}
        >
          {text("Button Label", "Click me! 👍")}
        </Button>
      </StoryWrapper>
    );
  });
