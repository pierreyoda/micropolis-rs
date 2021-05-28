import React from "react";
import { Meta } from "@storybook/react";

import "@/assets/styles/tailwind.css";
import MainLayout from "./main";

export default {
  title: "Layout",
  component: MainLayout,
} as Meta;

export const Main = () => (
  <MainLayout>
    <div className="w-full h-full bg-red-600">
      <h2>Main Layout</h2>
    </div>
  </MainLayout>
);
