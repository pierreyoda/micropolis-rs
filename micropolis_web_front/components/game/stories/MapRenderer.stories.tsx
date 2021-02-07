import React, { useMemo } from "react";
import { Meta } from "@storybook/react";
import { number } from "@storybook/addon-knobs";

import Tile from "../Tile";
import MapRenderer from "../MapRenderer";
import AtlasImage from "@/assets/game/tiles.png";
import { generateMapStub, getTestMap } from "@/game/utils";

export default {
  title: "Game/Map",
} as Meta;

export const CustomTile = () => (
  <Tile
    row={1}
    column={1}
    atlasImage={AtlasImage}
    tileIndex={number("Tile type", 0, { min: 0, max: 1018, step: 1 })}
  />
);

export const RandomMap = () => {
  const width = number("Map width", 25, { min: 5, max: 500, step: 5 });
  const height = number("Map height", 25, { min: 5, max: 500, step: 5 });
  const map = useMemo(() => generateMapStub(width, height), [width, height]);

  return map && <MapRenderer map={map} />;
};

const testMap = getTestMap();
export const TestMap = () => {
  return <MapRenderer map={testMap} />;
};
