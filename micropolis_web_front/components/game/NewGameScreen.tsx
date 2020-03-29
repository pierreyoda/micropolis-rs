import React, { FunctionComponent, useState } from "react";

import MapRenderer, { MapPayload, TileMeta } from "./MapRenderer";
import TextInput from "../common/TextInput";
import Button from "../common/Button";
import Card from "../common/Card";

export interface NewGameScreenProps {
}

// TODO: use real map generation WASM API
// const MAX_TILE_INDEX = 1028;
// const generateMap = (width: number, height: number): MapPayload => ({
//   tiles: [...Array(width).keys()].reduce((columns: TileMeta[][]) =>
//     [...columns,
//      ...Array(height).reduce((rows: TileMeta[]): TileMeta => [
//      ...rows, {
//        type: Math.round(Math.random() * MAX_TILE_INDEX),
//     }], []),
//   ], []
//   ),
// });

const NewGameScreen: FunctionComponent<NewGameScreenProps> = () => {
  const [cityName, setCityName] = useState("");

  const [generatedMaps, setGeneratedMaps] = useState<MapPayload[]>([]);

  return (
    <div className="flex items-start justify-center">
      <Card
        title={"New Game"}
        backgroundColor="#edad0a"
        >
        <TextInput
          value={cityName}
          onChange={setCityName}
          placeholder={"City name (mandatory)"}
        />
        <Button
          disabled={!cityName.length}
          onToggle={() => console.log("TODO: generate new map")}
        />
      </Card>
    </div>
  );
};

export default NewGameScreen;
