import React from "react";
import { NextPage } from "next";

import { MicropolisCoreLibConnector } from "@/game";
import NewGameScreen from "@/components/game/NewGameScreen";
import LoaderSpinner from "@/components/common/LoaderSpinner";
import GameCoreLibProvider, { GameCoreLibContext } from "@/components/game/GameCoreLibProvider";

const Home: NextPage = () => {
  // console.log("micropolis_core loaded", gameVersionInfo(gameLib));
  return (
    <GameCoreLibProvider>
      {/* <GameCoreLibContext.Consumer>
        {(lib: MicropolisCoreLibConnector) => (
          <h2>Rust WASM game core lib test: {lib.versionInfo}</h2>
        )}
      </GameCoreLibContext.Consumer> */}
      <div className="w-full h-full flex flex-col items-center justify-center">
        <NewGameScreen />
      </div>
    </GameCoreLibProvider>
  );
};

export default Home;
