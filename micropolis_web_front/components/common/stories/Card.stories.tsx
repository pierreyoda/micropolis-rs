import React, { useState } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, color, boolean, text } from "@storybook/addon-knobs";

import Card from "../Card";
import Button from "../Button";
import StoryWrapper from "./StoryWrapper";

storiesOf("Common", module)
  .addDecorator(withKnobs)
  .add("Card", () => {
    const [opened, setOpened] = useState(true);

    return (
      <StoryWrapper>
        {
          opened
            ?
              <Card
                title={text("Title", "Window UI title")}
                closable={boolean("Closable?", true)}
                onClose={() => setOpened(false)}
                backgroundColor={color("Background color", "#336075")}
              >
                <p className="py-6 text-white">
                  This is the Card's body.
                </p>
                <div className="pb-6">
                  <Button active={false} width="350px" onToggle={() => {}}>
                      Click me!
                  </Button>
                </div>
              </Card>
            :
              <Button active={false} onToggle={() => setOpened(true)}>
                Open
              </Button>
        }
        
      </StoryWrapper>
    );
  });
