import React, { FunctionComponent, useEffect, useState } from "react";

import Card from "../common/Card";
import Button from "../common/Button";
import TextInput from "../common/TextInput";
import { MicropolisCoreLibConnector } from "@/game";
import { GameMap, gameMapFromRawData } from "@/game/map";
import MapRenderer from "./MapRenderer";

export interface NewGameScreenProps {
  gameLib: MicropolisCoreLibConnector;
}

const NewGameScreen: FunctionComponent<NewGameScreenProps> = ({ gameLib }) => {
  const [cityName, setCityName] = useState("");
  const [currentlyViewedMapIndex, setCurrentlyViewedMapIndex] = useState(0);
  const [generatedMaps, setGeneratedMaps] = useState<readonly GameMap[]>([]);

  const generator = gameLib.createNewMapGenerator();
  const generateNewMap = () => {
    const rawMap = gameLib.generateNewRandomMap(generator, 120, 100);
    const map = gameMapFromRawData(rawMap);
    setGeneratedMaps(maps => [...maps, map]);
    setCurrentlyViewedMapIndex(generatedMaps.length);
  };
  useEffect(() => generateNewMap(), []);

  return (
    <div className="flex items-start justify-center">
      {generatedMaps.length > 0 && (
        <div className="flex flex-col mr-12">
          <MapRenderer scale={0.2} map={generatedMaps[currentlyViewedMapIndex]} />
          <div className="flex items-center justify-between w-full mt-4">
            <Button disabled={currentlyViewedMapIndex === 0} onToggle={() => setCurrentlyViewedMapIndex(i => i - 1)}>
              Previous
            </Button>
            <p className="text-center text-gray-700">
              {currentlyViewedMapIndex + 1} / {generatedMaps.length}
            </p>
            <Button
              disabled={currentlyViewedMapIndex >= generatedMaps.length - 1}
              onToggle={() => setCurrentlyViewedMapIndex(i => i + 1)}
            >
              Next
            </Button>
          </div>
        </div>
      )}
      <Card title="New Game" className="justify-between">
        <TextInput value={cityName} onChange={setCityName} placeholder="City name (mandatory)" />
        <div className="flex flex-col w-full">
          <Button onToggle={generateNewMap} className="w-full mt-10">
            Generate
          </Button>
          {generatedMaps.length > 0 && (
            <Button disabled={!cityName.length} onToggle={() => {}} className="w-full mt-4 bg-green-500">
              Play this map
            </Button>
          )}
        </div>
      </Card>
    </div>
  );
};

export default NewGameScreen;
