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

    return (
      <StoryWrapper>
        <span>{toggled ? "active" : "inactive"}</span>
        <Button
          disabled={boolean("Disabled?", false)}
          active={toggled}
          onToggle={() => setToggled(t => !t)}
          width={`${width}px`}
        >
          {text("Button Label", "Click me! 👍")}
        </Button>
      </StoryWrapper>
    );
  });
