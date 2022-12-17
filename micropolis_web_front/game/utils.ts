import { TileMeta, MapPayload } from "@/components/game/MapRenderer";
import TestMap from "@/public/utils-output/test-front-map";

export const testMapFromParsedJson = (parsedJson: readonly number[][]): MapPayload => {
  const tiles: TileMeta[][] = parsedJson.map((row): TileMeta[] =>
    row.map(
      (type): TileMeta => ({
        type,
      })
    )
  );
  return { tiles };
};

let cachedTestMap: MapPayload | undefined = undefined;
export const getTestMap = (): MapPayload => {
  if (cachedTestMap) {
    return cachedTestMap;
  }
  cachedTestMap = testMapFromParsedJson(TestMap);
  return cachedTestMap;
};

const MAX_TILE_INDEX = 1028;
export const generateMapStub = (width: number, height: number): MapPayload => ({
  tiles: [...Array(height).keys()].reduce(
    (columns: TileMeta[][]) => [
      ...columns,
      [...Array(width).keys()].reduce(
        (rows: TileMeta[]): TileMeta[] => [
          ...rows,
          {
            type: Math.round(Math.random() * MAX_TILE_INDEX),
          },
        ],
        []
      ),
    ],
    []
  ),
});
