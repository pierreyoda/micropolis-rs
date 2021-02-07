import React, { FunctionComponent } from "react";
import { Meta } from '@storybook/react/types-6-0';

import Toolbox, { toolboxActionsRegistry } from "../Toolbox";

export default {
  title: "Game/UI",
  component: Toolbox,
} as Meta;

export const MainToolbox: FunctionComponent = () => (
  <Toolbox actions={toolboxActionsRegistry.map((meta, i) => ({
    ...meta,
    onClick: () => { console.log(i + 1); },
  }))} />
);
