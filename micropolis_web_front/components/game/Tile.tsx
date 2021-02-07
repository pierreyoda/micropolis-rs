import React, { useMemo, FunctionComponent } from "react";
import { css } from "twin.macro";

export const TILE_SIZE = 16; // in pixels
const ATLAS_ROWS = 16;
const ATLAS_COLUMNS = 64;
const ATLAS_TILES = ATLAS_ROWS * ATLAS_COLUMNS;

export interface TileProps {
  row: number;
  column: number;
  tileIndex: number;
  atlasImage: string;
}

const Tile: FunctionComponent<TileProps> = ({ row, column, tileIndex, atlasImage }) => {
  const [atlasX, atlasY] = useMemo(
      () => [
        tileIndex % ATLAS_ROWS * TILE_SIZE,
        tileIndex / ATLAS_ROWS * TILE_SIZE,
      ],
      [tileIndex],
  );

  const [positionX, positionY] = useMemo(
    () => [row * TILE_SIZE, column * TILE_SIZE],
    [row, column],
  );

  return (
    <img
      src={atlasImage}
      style={{
        width: TILE_SIZE,
        height: TILE_SIZE,
        objectFit: "none",
        objectPosition: `-${atlasX}px -${atlasY}px`,
      }}
      css={css`
        user-select: none;
        -moz-user-select: none;
        -webkit-user-drag: none;
        -webkit-user-select: none;
        -ms-user-select: none;
      `}
    />
  );
};

export default Tile;
