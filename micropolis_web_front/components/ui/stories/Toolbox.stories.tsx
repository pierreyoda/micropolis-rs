import React, { FunctionComponent } from "react";
import { Meta } from '@storybook/react/types-6-0';

import Toolbox, { toolboxActionsRegistry } from "../Toolbox";
import StoryWrapper from "@/components/common/stories/StoryWrapper";

export const MainToolbox: FunctionComponent = () => (
  <StoryWrapper>
    <Toolbox actions={toolboxActionsRegistry.map((meta, i) => ({
      ...meta,
      onClick: () => { console.log(i + 1); },
    }))} />
  </StoryWrapper>
);

export default {
  title: "Game/UI/Toolbox",
  component: Toolbox,
} as Meta;
