export interface RawGameMap {
  /** Generation seed. */
  seed: number;
  /** Column-first. */
  map: {
    raw: number;
    tile_type?: string;
  }[][];
}

export interface GameTile {
  rawValue: number;
}

export interface GameMapTile {
  raw: number;
  tileType: number;
}

export interface GameMap {
  /** Column-first. */
  tiles: GameMapTile[][];
}

const TILE_TYPE_MASK = 0b0000_0011_1111_1111;
export const gameMapFromRawData = ({ map }: RawGameMap): GameMap => ({
  tiles: map.map(column => column.map((tile): GameMapTile => {
    const tileType = tile.raw & TILE_TYPE_MASK;
    return { raw: tile.raw, tileType };
  }))
});
