import React from "react";
import { Meta } from "@storybook/react";

import NewGameScreen, { generateMapStub } from "../NewGameScreen";

export default {
  title: "Game/Screens",
  component: NewGameScreen,
} as Meta;

export const NewGameScreenDemo = () => (
  <NewGameScreen
    generateMap={generateMapStub}
  />
);
