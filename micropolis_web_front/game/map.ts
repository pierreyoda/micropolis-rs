import { WebTileMap } from "@/pkg";

export interface RawGameMap {
  readonly handle: WebTileMap;
  /** Generation seed. */
  readonly seed: number;
  /** Column-first. */
  readonly map: {
    readonly raw: number;
    readonly tile_type?: string;
  }[][];
}

export interface GameTile {
  readonly rawValue: number;
}

export interface GameMapTile {
  readonly raw: number;
  readonly tileType: number;
}

export interface GameMap {
  /** Column-first. */
  readonly tiles: readonly GameMapTile[][];
}

const TILE_TYPE_MASK = 0b0000_0011_1111_1111;
export const gameMapFromRawData = ({ map }: RawGameMap): GameMap => ({
  tiles: map.map(column =>
    column.map(
      (tile): GameMapTile => {
        const tileType = tile.raw & TILE_TYPE_MASK;
        return { raw: tile.raw, tileType };
      }
    )
  ),
});
