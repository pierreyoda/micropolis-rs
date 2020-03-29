import React, { useState } from "react";
import { NextPage } from "next";

import { MicropolisCoreLibConnector } from "@/game";
import NewGameScreen from "@/components/game/NewGameScreen";
import LoaderSpinner from "@/components/common/LoaderSpinner";
import GameCoreLibProvider, { GameCoreLibContext } from "@/components/game/GameCoreLibProvider";

const Home: NextPage = () => {
  const [loading, setLoading] = useState(true);
  const [gameLib, setGameLib] = useState<MicropolisCoreLibConnector | null>(null);
  import("@/pkg").then(async module => {
    const connector = new MicropolisCoreLibConnector(module);
    setGameLib(connector);
    await new Promise(resolve => setTimeout(resolve, 1000)); // TODO: test
    // setLoading(false);
  });
  return (
    <GameCoreLibContext.Provider value={gameLib}>
      <div className="w-full h-full flex flex-col items-center justify-center">
        {loading
          ? <div className="w-full h-full flex flex-col items-center justify-center">
              <LoaderSpinner width={250} height={250} type="MutatingDots" />
              <span className="mt-12">Loading game library...</span>
            </div>
          : <NewGameScreen />
        }
      </div>
    </GameCoreLibContext.Provider>
  );
};

export default Home;
