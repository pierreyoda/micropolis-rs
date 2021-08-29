import { FunctionComponent } from "react";

import { WebCity } from "@/pkg";
import { MicropolisCoreLibConnector } from "@/game";

interface GameScreenProps {
  playedCity: WebCity;
  gameLib: MicropolisCoreLibConnector;
}

export const GameScreen: FunctionComponent<GameScreenProps> = ({ gameLib, playedCity }) => {
  return <div className="w-full h-full bg-red-50"></div>;
};
