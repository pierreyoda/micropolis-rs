import React from "react";
import { Meta } from "@storybook/react";

import NewGameScreen from "../NewGameScreen";
import { generateMapStub } from "@/game/utils";

export default {
  title: "Game/Screens",
  component: NewGameScreen,
} as Meta;

export const NewGameScreenDemo = () => <NewGameScreen generateMap={generateMapStub} />;
