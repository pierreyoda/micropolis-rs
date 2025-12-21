import dynamic from "next/dynamic";
import { createContext } from "react";

import { connectMicropolisCoreLib, MicropolisCoreLibConnector, importMicropolisCoreWasmLib } from "@/game";

export const GameCoreLibContext = createContext<MicropolisCoreLibConnector | null>(null);
GameCoreLibContext.displayName = "MicropolisGameCoreLib";

const GameCoreLibProvider = dynamic({
  loading: () => <span>Loading game lib...</span>,
  loader: async () => {
    const coreLib = await importMicropolisCoreWasmLib();
    const gameCoreLib = connectMicropolisCoreLib(coreLib);
    const GameCoreLibContextProvider = () => <GameCoreLibContext.Provider value={gameCoreLib} />;
    return GameCoreLibContextProvider;
  },
  ssr: false,
});

export default GameCoreLibProvider;
