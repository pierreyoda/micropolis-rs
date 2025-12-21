import { Meta } from "@storybook/react";
import { FunctionComponent, useState } from "react";

import { ToolboxActionID, toolboxActionsIDs } from "@/game/toolbox";
import Toolbox, { toolboxActionsRegistry } from "../Toolbox";

export default {
  title: "Game/UI/Toolbox",
  component: Toolbox,
} as Meta;

export const MainToolbox: FunctionComponent = () => {
  const [selected, setSelected] = useState<ToolboxActionID | null>(null);

  return (
    <div className="flex items-center justify-center w-screen h-screen bg-red-100">
      <Toolbox
        selected={selected}
        onSelection={newSelected => {
          setSelected(newSelected);
          console.log(newSelected);
        }}
        actions={toolboxActionsIDs.map(actionID => ({
          ...toolboxActionsRegistry[actionID],
          actionID,
        }))}
      />
    </div>
  );
};
