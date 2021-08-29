import React, { useState, useEffect } from "react";
import { NextPage } from "next";

import { WebCity } from "@/pkg";
import { MicropolisCoreLibConnector } from "@/game";
import { GameScreen } from "@/components/game/GameScreen";
import NewGameScreen from "@/components/game/NewGameScreen";
import LoaderSpinner from "@/components/common/LoaderSpinner";

const Home: NextPage = () => {
  const [loading, setLoading] = useState(true);
  const [gameLib, setGameLib] = useState<MicropolisCoreLibConnector | null>(null);
  useEffect(() => {
    const loadCoreLibrary = async () => {
      const coreModule = await import(/* webpackMode: "lazy" */ "../pkg/");
      const coreConnector = new MicropolisCoreLibConnector(coreModule);
      setGameLib(coreConnector);
      setLoading(false);
    };
    loadCoreLibrary();
  }, []);

  const [playedCity, setPlayedCity] = useState<WebCity | null>(null);
  console.log("playedCity", playedCity);

  return (
    <div className="flex flex-col items-center justify-center w-full h-full">
      {loading ? (
        <div className="flex flex-col items-center justify-center w-full h-full">
          <LoaderSpinner width={250} height={250} type="MutatingDots" />
          <span className="mt-12">Loading game library...</span>
        </div>
      ) : playedCity ? (
        <GameScreen gameLib={gameLib!} playedCity={playedCity} />
      ) : (
        <NewGameScreen gameLib={gameLib!} onCityCreated={setPlayedCity} />
      )}
    </div>
  );
};

export default Home;
