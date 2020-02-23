import React from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, color, boolean, text } from "@storybook/addon-knobs";

import Card from "../Card";
import Button from "../Button";
import StoryWrapper from "./StoryWrapper";

storiesOf("Common", module)
  .addDecorator(withKnobs)
  .add("Card", () => {
    return (
      <StoryWrapper>
        <Card
          title={text("Title", "Window UI title")}
          closable={boolean("Closable?", false)}
          backgroundColor={color("Background color", "#ccffae")}
        >
          <p className="py-6">
            This is the Card's body.
          </p>
          <Button active={false} width="350px" onToggle={() => {}}>
              Click me!
          </Button>
        </Card>
      </StoryWrapper>
    );
  });
