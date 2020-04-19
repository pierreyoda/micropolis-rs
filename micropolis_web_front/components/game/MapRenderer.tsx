import React, { FunctionComponent } from "react";

import Tile from "./Tile";
import AtlasImage from "@/assets/game/tiles.png";

export interface TileMeta {
  type: number;
}

export interface MapPayload {
  tiles: TileMeta[][];
}

export interface MapRendererProps {
  map: MapPayload;
}

/**
 * TileMap renderer.
 */
const MapRenderer: FunctionComponent<MapRendererProps> = ({
  map: { tiles },
}) => {
  return (
    <div className="flex">
      {tiles.map((col, colIndex) => (
        <div className="flex-col" key={colIndex}>
          {col.map(({ type }, rowIndex) => <Tile
            key={rowIndex}
            row={rowIndex}
            column={colIndex}
            tileIndex={type}
            atlasImage={AtlasImage}
          />)}
        </div>
      ))}
    </div>
  );
}

export default MapRenderer;
