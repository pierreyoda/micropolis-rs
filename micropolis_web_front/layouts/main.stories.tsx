import React from "react";
import { storiesOf } from "@storybook/react";

import "@/assets/styles/tailwind.css";
import MainLayout from "./main";

storiesOf("Layout", module)
  .add("Main", () => (
    <MainLayout>
      <div className="w-full h-full bg-red-600">
        <h2>Main Layout</h2>
      </div>
    </MainLayout>
  ));
