import React from "react";
import { storiesOf } from "@storybook/react";
import { withKnobs } from "@storybook/addon-knobs";

import Toolbar, { ToolbarItem, ToolbarItemChild } from "../Toolbar";
import StoryWrapper from "@/components/common/stories/StoryWrapper";

const buildChildren = (count: number): ToolbarItemChild[] =>
  [...Array(count).keys()].map((i): ToolbarItemChild => ({
    key: `child-${i}`,
    label: `Sub-item ${i + 1}`,
  }));

storiesOf("Game/UI", module)
    .addDecorator(withKnobs)
    .add("Toolbar", () => {
      const items: ToolbarItem[] = [
        { 
          key: "main",
          label: "Micropolis",
          children: [],
        },
        { 
          key: "options",
          label: "Options",
          children: [],
        },
        { 
          key: "disasters",
          label: "Disasters",
          children: [],
        },
        { 
          key: "time",
          label: "Time",
          children: [],
        },
        {
          key: "priority",
          label: "Priority",
          children: [],
        },
        { 
          key: "windows",
          label: "Windows",
          children: [],
        },
      ].map(item => ({
        ...item,
        children: buildChildren(6),
      }));
      console.log(buildChildren(5));

      return (
        <StoryWrapper>
          <Toolbar items={items} />
        </StoryWrapper>
      );
    });
