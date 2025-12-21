import { useMemo, FunctionComponent } from "react";

export const TILE_SIZE = 16; // in pixels
const ATLAS_ROWS = 16;
const ATLAS_COLUMNS = 64;
const ATLAS_TILES = ATLAS_ROWS * ATLAS_COLUMNS;

export interface TileProps {
  row: number;
  column: number;
  tileIndex: number;
  atlasImageUrl: string;
  scale: number;
}

const Tile: FunctionComponent<TileProps> = ({ tileIndex, atlasImageUrl, scale }) => {
  const [atlasX, atlasY] = useMemo(() => [(tileIndex % ATLAS_ROWS) * TILE_SIZE, (tileIndex / ATLAS_ROWS) * TILE_SIZE], [
    tileIndex,
  ]);

  return (
    <img
      src={atlasImageUrl}
      style={{
        width: TILE_SIZE * scale,
        height: TILE_SIZE * scale,
        objectFit: "none",
        objectPosition: `-${atlasX}px -${atlasY}px`,
      }}
      className="not-selectable"
    />
  );
};

export default Tile;
