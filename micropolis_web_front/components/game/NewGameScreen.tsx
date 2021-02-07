import React, { FunctionComponent, useState } from "react";
import "twin.macro";

import MapRenderer, { MapPayload, TileMeta } from "./MapRenderer";
import TextInput from "../common/TextInput";
import Button from "../common/Button";
import Card from "../common/Card";

export interface NewGameScreenProps {
  generateMap: (width: number, height: number) => MapPayload;
}

const NewGameScreen: FunctionComponent<NewGameScreenProps> = () => {
  const [cityName, setCityName] = useState("");

  const [generatedMaps, setGeneratedMaps] = useState<MapPayload[]>([]);

  return (
    <div tw="flex items-start justify-center">
      <Card title={"New Game"}>
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
