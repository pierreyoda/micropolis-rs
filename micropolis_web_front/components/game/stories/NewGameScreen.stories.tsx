import React from "react";
import { storiesOf } from "@storybook/react";

import NewGameScreen, { generateMapStub } from "../NewGameScreen";
import StoryWrapper from "@/components/common/stories/StoryWrapper";

storiesOf("Game/Screens", module)
  .add("New Game Screen", () => (
    <StoryWrapper full>
      <NewGameScreen
        generateMap={generateMapStub}
      />
    </StoryWrapper>
  ));
