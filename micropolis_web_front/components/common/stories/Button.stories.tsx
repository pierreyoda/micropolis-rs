import React, { useState } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, text, boolean } from "@storybook/addon-knobs";

import Button from "../Button";
import StoryWrapper from "./StoryWrapper";

storiesOf("Common", module)
  .addDecorator(withKnobs)
  .add("Button", () => {
    const [toggled, setToggled] = useState(false);
    return (
      <StoryWrapper>
        <span>{toggled ? "active" : "inactive"}</span>
        <Button
          disabled={boolean("Disabled?", false)}
          onToggle={() => setToggled(t => !t)}
        >
          {text("Button Label", "Click me! 👍")}
        </Button>
      </StoryWrapper>
    );
  });
