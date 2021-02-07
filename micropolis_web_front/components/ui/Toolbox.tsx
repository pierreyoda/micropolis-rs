import React, { useMemo, FunctionComponent } from "react";
import "twin.macro";

import { iterate_by_pairs } from "@/utils";

export interface ToolboxActionProps {
  iconImg: string;
  tooltip: string;
  onClick: () => void;
}

export interface ToolboxProps {
  actions: readonly ToolboxActionProps[];
}

const ToolboxAction: FunctionComponent<ToolboxActionProps> = ({
  iconImg: iconSrc, tooltip, onClick,
}) => (
  <div tw="w-12 h-12" onClick={onClick}>
    <img
      alt={tooltip}
      title={tooltip}
      src={useMemo(() => `/game/toolbox/${iconSrc}`, [iconSrc])}
      tw="border-2 rounded-sm border-transparent hover:border-blue-400"
    />
  </div>
);

export const toolboxActionsRegistry: readonly (Omit<ToolboxActionProps, "onClick">)[] = [
  { iconImg: "bulldozer.png", tooltip: "Bulldozer Tool" },
  { iconImg: "eraser.png", tooltip: "Eraser Tool" },
  { iconImg: "wire.png", tooltip: "Wire Tool" },
  { iconImg: "road.png", tooltip: "Road Tool" },
  { iconImg: "rail.png", tooltip: "Rail Tool" },
  { iconImg: "query.png", tooltip: "Query Tool" },
  { iconImg: "residential.png", tooltip: "Residential Zone" },
  { iconImg: "commercial.png", tooltip: "Commercial Zone" },
  { iconImg: "industrial.png", tooltip: "Industrial Zone" },
  { iconImg: "firestation.png", tooltip: "Fire Station" },
  { iconImg: "policestation.png", tooltip: "Police Station" },
  { iconImg: "coal.png", tooltip: "Coal Power Plant" },
  { iconImg: "nuclear.png", tooltip: "Nuclear Power Plant" },
  { iconImg: "airport.png", tooltip: "Airport" },
  { iconImg: "seaport.png", tooltip: "Seaport" },
  { iconImg: "stadium.png", tooltip: "Stadium" },
];

const Toolbox: FunctionComponent<ToolboxProps> = ({
  actions,
}) => {
  const actionsPairs = useMemo(() => [
    ...iterate_by_pairs(actions),
  ], [actions]);

  return (
    <div tw="flex flex-col w-16 pt-4 bg-gray-100">
      {actionsPairs.map(([firstAction, secondAction]) => (
        <div
          key={`${firstAction.iconImg}${secondAction ? `+${secondAction.iconImg}` : ""}`}
          tw="flex flex-row">
          <ToolboxAction {...firstAction} />
          {secondAction && <ToolboxAction {...secondAction} />}
        </div>
      ))}
    </div>
  );
};

export default Toolbox;
