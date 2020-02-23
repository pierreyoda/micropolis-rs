import React, { createContext } from "react";
import dynamic from "next/dynamic";

import { importMicropolisCoreWasmLib, connectMicropolisCoreLib } from "@/game";

export const GameCoreLibContext = createContext(undefined);
GameCoreLibContext.displayName = "MicropolisGameCoreLib";

const GameCoreLibProvider = dynamic({
  loading: () => <span>Loading game lib...</span>,
  loader: async () => {
    const gameCoreLib = connectMicropolisCoreLib(await importMicropolisCoreWasmLib());
    // const GameCoreLibContextProvider = () => (
    //   <GameCoreLibContext.Provider value={gameCoreLib} />
    // );
    return ({ children }) => (
      <div>
        {children}
      </div>
    );
    // return GameCoreLibContextProvider;
  },
  ssr: false,
  // loading: () => "Loading..."
});

export default GameCoreLibProvider;
