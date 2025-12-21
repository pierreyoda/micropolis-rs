import { useState, useEffect } from "react";
import { NextPage } from "next";

import NewGameScreen from "@/components/game/NewGameScreen";
import LoaderSpinner from "@/components/common/LoaderSpinner";
import { importMicropolisCoreWasmLib, MicropolisCoreLibConnector } from "@/game";

const Home: NextPage = () => {
  const [loading, setLoading] = useState(true);
  const [gameLib, setGameLib] = useState<MicropolisCoreLibConnector | null>(null);
  useEffect(() => {
    const loadCoreLibrary = async () => {
      const coreModule = await importMicropolisCoreWasmLib();
      const coreConnector = new MicropolisCoreLibConnector(coreModule);
      setGameLib(coreConnector);
      setLoading(false);
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
