import React from "react";
import { NextPage } from "next";
import dynamic from "next/dynamic";

import LoaderSpinner from "@/components/common/LoaderSpinner";
import GameCoreLibProvider, { GameCoreLibContext } from "@/components/game/GameCoreLibProvider";
import { MicropolisCoreLibConnector } from "@/game";

const PixiContainerLoader = () => (
  <div className="w-full h-full flex flex-col items-center justify-center p-12">
    <LoaderSpinner type="Watch" />
  </div>
);

const PixiContainer = dynamic(
  () => import("@/components/game/PixiContainer"),
  { ssr: false, loading: PixiContainerLoader },
);

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
        <PixiContainer debug />
      </div>
    </GameCoreLibProvider>
  );
};

export default Home;
