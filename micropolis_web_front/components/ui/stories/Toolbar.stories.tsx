import { Meta } from "@storybook/react";

import Toolbar, { ToolbarItem, ToolbarItemChild } from "../Toolbar";

const buildChildren = (count: number): ToolbarItemChild[] =>
  [...Array(count).keys()].map(
    (i): ToolbarItemChild => ({
      key: `child-${i}`,
      label: `Sub-item ${i + 1}`,
      onClick: () => {
        console.log(`child-${i}`);
      },
    })
  );

export default {
  title: "Game/UI/Toolbar",
  component: Toolbar,
} as Meta;

export const ToolbarDemo = () => {
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

  return <Toolbar items={items} />;
};
