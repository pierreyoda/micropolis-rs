import React, { useMemo } from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs, number } from "@storybook/addon-knobs";

import Tile from "../Tile";
import MapRenderer from "../MapRenderer";
import AtlasImage from "@/assets/game/tiles.png";
import StoryWrapper from "@/components/common/stories/StoryWrapper";
import { generateMapStub } from "../NewGameScreen";

storiesOf("Game/Map", module)
  .addDecorator(withKnobs)
  .add("Custom Tile", () => (
    <StoryWrapper>
      <Tile
        row={1}
        column={1}
        atlasImage={AtlasImage}
        tileIndex={number("Tile type", 0, { min: 0, max: 1018, step: 1 })}
      />
    </StoryWrapper>
  ))
  .add("Map Renderer", () => {
    const width = number("Map width", 50, { min: 5, max: 500, step: 5 });
    const height = number("Map height", 50, { min: 5, max: 500, step: 5 });
    const map = useMemo(() => generateMapStub(width, height), [width, height]);

    return (
      <StoryWrapper>
        {map && <MapRenderer map={map} />}
      </StoryWrapper>
    );
  })
