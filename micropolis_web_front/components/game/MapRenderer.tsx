import React, { FunctionComponent } from "react";

import Tile from "./Tile";
import { GameMap } from "@/game/map";

export interface MapRendererProps {
  map: GameMap;
  /** Between 0 and 1. */
  scale?: number;
}

/**
 * TileMap renderer.
 */
const MapRenderer: FunctionComponent<MapRendererProps> = ({
  map: { tiles },
  scale,
}) => {
  return (
    <div className="flex">
      {tiles.map((col, colIndex) => (
        <div className="flex-col" key={colIndex}>
          {col.map(({ tileType }, rowIndex) => (
            <Tile
              key={rowIndex}
              row={rowIndex}
              column={colIndex}
              tileIndex={tileType > 0 ? tileType : 0} // dirt for invalid
              atlasImageUrl={"/game/tiles.png"}
              scale={scale ?? 1}
            />
          ))}
        </div>
      ))}
    </div>
  );
};

export default MapRenderer;
