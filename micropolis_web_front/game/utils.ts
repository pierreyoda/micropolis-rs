import { TileMeta, MapPayload } from "@/components/game/MapRenderer";
import TestMap from "@/public/utils-output/test-front-map.json";

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
