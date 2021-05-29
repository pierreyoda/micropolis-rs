import { FunctionComponent, useState, useMemo, useEffect } from "react";

import Card from "../common/Card";
import Button from "../common/Button";
import MapRenderer from "./MapRenderer";
import TextInput from "../common/TextInput";
import { MicropolisCoreLibConnector } from "@/game";
import { GameMap, gameMapFromRawData } from "@/game/map";
import { getRandomInt } from "@/utils";

export interface NewGameScreenProps {
  gameLib: MicropolisCoreLibConnector;
}

const generateSeed = (): number => getRandomInt(123, 123456);

interface GeneratedGameMap {
  seed: number;
  gameMap: GameMap;
}

const NewGameScreen: FunctionComponent<NewGameScreenProps> = ({ gameLib }) => {
  const [cityName, setCityName] = useState("");
  const [currentlyViewedMapIndex, setCurrentlyViewedMapIndex] = useState(0);
  const [generatedMaps, setGeneratedMaps] = useState<readonly GeneratedGameMap[]>([]);

  const generator = gameLib.createNewMapGenerator();
  const generateNewMap = () => {
    const rawMap = gameLib.generateNewRandomMap(generator, generateSeed(), 120, 100);
    const gameMap = gameMapFromRawData(rawMap);
    setGeneratedMaps(generatedMaps => [...generatedMaps, { gameMap, seed: rawMap.seed }]);
    setCurrentlyViewedMapIndex(generatedMaps.length);
  };
  useEffect(() => generateNewMap(), []);

  const curentGeneratedMap = useMemo(() => generatedMaps[currentlyViewedMapIndex], [
    generatedMaps,
    currentlyViewedMapIndex,
  ]);

  return (
    <div className="flex items-start justify-center">
      {curentGeneratedMap && (
        <div className="flex flex-col mr-12">
          <p className="mb-4 text-center text-gray-700">Seed: {curentGeneratedMap.seed}</p>
          <MapRenderer scale={0.2} map={curentGeneratedMap.gameMap} />
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
          {curentGeneratedMap && (
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
