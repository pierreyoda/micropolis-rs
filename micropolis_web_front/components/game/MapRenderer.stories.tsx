import React from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, number } from "@storybook/addon-knobs";

import Tile from "./Tile";
import AtlasImage from "@/assets/game/tiles.png";
import StoryWrapper from "../common/stories/StoryWrapper";

storiesOf("Game/Map", module)
  .addDecorator(withKnobs)
  .add("Custom Tile", () => (
    <StoryWrapper>
      <Tile
        row={1}
        column={1}
        tileIndex={number("Tile type", 0, { min: 0, max: 1018, step: 1 })}
        atlasImage={AtlasImage}
      />
    </StoryWrapper>
  ));
