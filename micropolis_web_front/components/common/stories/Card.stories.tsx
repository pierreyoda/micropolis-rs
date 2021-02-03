import React, { useState } from "react";
import { Meta } from "@storybook/react";

import Card from "../Card";
import Button from "../Button";

export const Closable = () => {
  const [opened, setOpened] = useState(true);

  return opened
    ? (
      <Card
        closable
        title="Title"
        onClose={() => setOpened(false)}
      >
        <p className="py-6 text-white">
          This is the Card's body.
        </p>
        <div className="pb-6">
          <Button onToggle={() => {}}>
            Click me!
          </Button>
        </div>
      </Card>
    ) : (
      <Button onToggle={() => setOpened(true)}>
        Open
      </Button>
    );
};

export default {
  title: "Card",
  component: Card,
} as Meta;
