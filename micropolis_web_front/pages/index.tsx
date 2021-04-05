import React, { useState, useEffect } from "react";
import { NextPage } from "next";

import { MicropolisCoreLibConnector } from "@/game";
import NewGameScreen from "@/components/game/NewGameScreen";
import LoaderSpinner from "@/components/common/LoaderSpinner";
// import { GameCoreLibContext } from "@/components/game/GameCoreLibProvider";

const Home: NextPage = () => {
  const [loading, setLoading] = useState(true);
  const [gameLib, setGameLib] = useState<MicropolisCoreLibConnector | null>(
    null
  );
  useEffect(() => {
    const loadCoreLibrary = async () => {
      const coreModule = await import(/* webpackMode: "lazy" */ "../pkg/");
      const coreConnector = new MicropolisCoreLibConnector(coreModule);
      setGameLib(coreConnector);
      setLoading(false);
      const generator = coreConnector.createNewMapGenerator();
      const map = coreConnector.generateNewRandomMap(generator, 120, 100);
      console.log(map);
    };
    loadCoreLibrary();
  }, []);

  return (
    // <GameCoreLibContext.Provider value={gameLib}>
    <div className="flex flex-col items-center justify-center w-full h-full">
      {loading ? (
        <div className="flex flex-col items-center justify-center w-full h-full">
          <LoaderSpinner width={250} height={250} type="MutatingDots" />
          <span className="mt-12">Loading game library...</span>
        </div>
      ) : (
        <NewGameScreen gameLib={gameLib!} />
      )}
    </div>
    // </GameCoreLibContext.Provider>
  );
};

export default Home;
