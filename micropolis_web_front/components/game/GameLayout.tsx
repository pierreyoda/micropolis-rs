import { FunctionComponent, ReactNode, useState } from "react";

import Toolbar, { ToolbarItem } from "@/components/ui/Toolbar";
import Toolbox, { toolboxActions } from "@/components/ui/Toolbox";

interface GameLayoutProps {
  children: ReactNode;
}

// TODO: add `onClick`s
const TOOLBAR_ITEMS: readonly ToolbarItem[] = [
  {
    key: "micropolis",
    label: "Micropolis",
    children: [
      {
        key: "micropolis-about",
        label: "About",
      },
      {
        key: "micropolis-save-city",
        label: "Save City",
      },
      {
        key: "micropolis-save-city-as",
        label: "Save City As",
      },
    ],
  },
  {
    key: "disasters",
    label: "Disasters",
    children: [
      {
        key: "disasters-monster",
        label: "Monster",
      },
      {
        key: "disasters-fire",
        label: "Fire",
      },
      {
        key: "disasters-flood",
        label: "Flood",
      },
      {
        key: "disasters-meltdown",
        label: "Meltdown",
      },
      {
        key: "disasters-tornado",
        label: "Tornado",
      },
      {
        key: "disasters-earthquake",
        label: "Earthquake",
      },
    ],
  },
  {
    key: "windows",
    label: "Windows",
    children: [
      {
        key: "windows-budget",
        label: "Budget",
      },
      {
        key: "windows-evaluation",
        label: "Evaluation",
      },
      {
        key: "windows-graph",
        label: "Graph",
      },
    ],
  },
];

export const GameLayout: FunctionComponent<GameLayoutProps> = ({ children }) => {
  const [selectedTool, setSelectedTool] = useState<ToolboxActionID | null>(null);

  return (
    <div className="flex flex-col w-full h-full">
      <Toolbar items={TOOLBAR_ITEMS} />
      <div className="flex">
        <Toolbox selected={selectedTool} onSelection={setSelectedTool} actions={toolboxActions} />
        {children}
      </div>
    </div>
  );
};
