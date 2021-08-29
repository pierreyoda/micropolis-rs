import { Meta } from "@storybook/react";

import { GameLayout } from "../GameLayout";

export const Default = () => (
  <div className="flex items-center justify-center w-screen h-screen bg-red-100">
    <GameLayout>
      <div className="flex items-center justify-center w-full h-full">Game Screen</div>
    </GameLayout>
  </div>
);

export default {
  title: "Game/UI/Layout",
  component: GameLayout,
} as Meta;
