import { NextPage } from "next";
import React, { useState, useEffect } from "react";
import { MutatingDots } from "react-loader-spinner";

import { MicropolisCoreLibConnector } from "@/game";
import NewGameScreen from "@/components/game/NewGameScreen";

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

  return (
    <div className="flex flex-col items-center justify-center w-full h-full">
      {loading ? (
        <div className="flex flex-col items-center justify-center w-full h-full">
          <MutatingDots width={250} height={250} />
          <span className="mt-12">Loading game library...</span>
        </div>
      ) : (
        <NewGameScreen gameLib={gameLib!} />
      )}
    </div>
  );
};

export default Home;
