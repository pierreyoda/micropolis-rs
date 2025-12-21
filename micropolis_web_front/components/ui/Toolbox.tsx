import clsx from "clsx";
import { useMemo, FunctionComponent } from "react";

import { iterate_by_pairs } from "@/utils";
import { ToolboxActionID, toolboxActionsIDs } from "@/game/toolbox";

interface ToolboxActionProps {
  actionID: ToolboxActionID;
  iconImg: string;
  tooltip: string;
  onClick: () => void;
  selected: boolean;
}

const ToolboxAction: FunctionComponent<ToolboxActionProps> = ({ iconImg: iconSrc, tooltip, onClick, selected }) => (
  <div
    className={clsx(
      "w-12 h-auto mb-2 mr-1 border-4 shadow-toolbar last:mr-0 hover:border-blue-400",
      selected ? "border-blue-500" : "border-gray-500"
    )}
    onClick={onClick}
  >
    <img
      alt={tooltip}
      title={tooltip}
      src={useMemo(() => `/game/toolbox/${iconSrc}`, [iconSrc])}
      className="w-full h-full rounded-sm"
    />
  </div>
);

export const toolboxActionsRegistry: Record<ToolboxActionID, Omit<ToolboxActionProps, "actionID" | "selected" | "onClick">> = {
  bulldozer: { iconImg: "bulldozer.png", tooltip: "Bulldozer Tool" },
  eraser: { iconImg: "eraser.png", tooltip: "Eraser Tool" },
  wire: { iconImg: "wire.png", tooltip: "Wire Tool" },
  road: { iconImg: "road.png", tooltip: "Road Tool" },
  rail: { iconImg: "rail.png", tooltip: "Rail Tool" },
  query: { iconImg: "query.png", tooltip: "Query Tool" },
  residential: { iconImg: "residential.png", tooltip: "Residential Zone" },
  commercial: { iconImg: "commercial.png", tooltip: "Commercial Zone" },
  industrial: { iconImg: "industrial.png", tooltip: "Industrial Zone" },
  firestation: { iconImg: "firestation.png", tooltip: "Fire Station" },
  policestation: { iconImg: "policestation.png", tooltip: "Police Station" },
  coal: { iconImg: "coal.png", tooltip: "Coal Power Plant" },
  nuclear: { iconImg: "nuclear.png", tooltip: "Nuclear Power Plant" },
  airport: { iconImg: "airport.png", tooltip: "Airport" },
  seaport: { iconImg: "seaport.png", tooltip: "Seaport" },
  stadium: { iconImg: "stadium.png", tooltip: "Stadium" },
};
export const toolboxActions = toolboxActionsIDs.map(actionID => toolboxActionsRegistry[actionID]);

interface ToolboxProps {
  selected: ToolboxActionID | null;
  onSelection: (newSelected: ToolboxActionID) => void;
  actions: readonly Omit<ToolboxActionProps, "onClick" | "selected">[];
}

const Toolbox: FunctionComponent<ToolboxProps> = ({ selected, onSelection, actions }) => {
  const actionsPairs = useMemo(() => [...iterate_by_pairs(actions)], [actions]);
  const onActionClick = (action: Omit<ToolboxActionProps, "onClick" | "selected">) => () => {
    onSelection(action.actionID);
  };

  return (
    <div className="flex flex-col w-32 pt-4 bg-gray-400 border-4 border-gray-500 rounded-sm shadow-toolbar">
      {actionsPairs.map(([firstAction, secondAction]) => (
        <div
          key={`${firstAction.iconImg}${secondAction ? `+${secondAction.iconImg}` : ""}`}
          className="flex flex-row justify-center w-full"
        >
          <ToolboxAction
            {...firstAction}
            selected={firstAction.actionID === selected}
            onClick={onActionClick(firstAction)}
          />
          {secondAction && (
            <ToolboxAction
              {...secondAction}
              selected={secondAction.actionID === selected}
              onClick={onActionClick(secondAction)}
            />
          )}
        </div>
      ))}
    </div>
  );
};

export default Toolbox;
