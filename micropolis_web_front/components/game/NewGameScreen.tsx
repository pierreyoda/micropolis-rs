import { FunctionComponent, useState, useMemo, useEffect } from "react";

import { getRandomInt } from "@/utils";
import { WebCity, WebTileMap } from "@/pkg";
import { MicropolisCoreLibConnector } from "@/game";
import { GameMap, gameMapFromRawData } from "@/game/map";
import Card from "../common/Card";
import Button from "../common/Button";
import MapRenderer from "./MapRenderer";
import TextInput from "../common/TextInput";

interface NewGameScreenProps {
  gameLib: MicropolisCoreLibConnector;
  onCityCreated: (city: WebCity) => void;
}

const generateSeed = (): number => getRandomInt(123, 123456);

interface GeneratedGameMap {
  seed: number;
  gameMap: GameMap;
  gameMapHandle: WebTileMap;
}

const NewGameScreen: FunctionComponent<NewGameScreenProps> = ({ gameLib, onCityCreated }) => {
  const [cityName, setCityName] = useState("");
  const [currentlyViewedMapIndex, setCurrentlyViewedMapIndex] = useState(0);
  const [generatedMaps, setGeneratedMaps] = useState<readonly GeneratedGameMap[]>([]);

  const reset = () => {
    setCityName("");
    setCurrentlyViewedMapIndex(0);
    setGeneratedMaps([]);
  };

  // city map generation
  const cityGenerator = useMemo(() => {
    const builder = gameLib.createNewCityGeneratorBuilder();
    return builder.with_city_map_generator_options(120, 100, true).build();
  }, [gameLib]);
  const generateNewMap = () => {
    const rawMap = gameLib.generateNewRandomMap(cityGenerator, generateSeed());
    const gameMap = gameMapFromRawData(rawMap);
    setGeneratedMaps(maps => [
      ...maps,
      {
        gameMap,
        gameMapHandle: rawMap.handle,
        seed: rawMap.seed,
      },
    ]);
    setCurrentlyViewedMapIndex(generatedMaps.length);
  };
  useEffect(() => generateNewMap(), []);

  const currentGeneratedMap: GeneratedGameMap | undefined = useMemo(() => generatedMaps[currentlyViewedMapIndex], [
    generatedMaps,
    currentlyViewedMapIndex,
  ]);

  // game state handling
  const cityNameIsValid = cityName.trim().length > 0;
  const startGame = () => {
    if (!cityNameIsValid || !currentGeneratedMap) {
      return;
    }
    const city = cityGenerator.generate(cityName, currentGeneratedMap.gameMapHandle);
    console.log("city", city);
    onCityCreated(city);
    reset();
  };

  return (
    <div className="flex items-center justify-center">
      {currentGeneratedMap && (
        <div className="flex flex-col mr-12">
          <p className="mb-4 text-center text-gray-700">Seed: {currentGeneratedMap.seed}</p>
          <div className="border-4 border-gray-500 rounded">
            <MapRenderer scale={0.2} map={currentGeneratedMap.gameMap} />
          </div>
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
      <Card title="New Game" className="justify-between bg-blue-50">
        <TextInput value={cityName} onChange={setCityName} placeholder="City name (mandatory)" />
        <div className="flex flex-col w-full">
          <Button onToggle={generateNewMap} className="w-full mt-10">
            Generate
          </Button>
          {currentGeneratedMap && (
            <Button disabled={!cityNameIsValid} onToggle={startGame} className="w-full mt-4 bg-green-500">
              Play this map
            </Button>
          )}
        </div>
      </Card>
    </div>
  );
};

export default NewGameScreen;
