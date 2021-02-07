import React, { FunctionComponent } from "react";
import "twin.macro";

import Button from "./Button";

export interface CardProps {
  title: string;
  closable?: boolean;
  onClose?: () => void;
}

const Card: FunctionComponent<CardProps> = ({
  children,
  title,
  closable = false,
  onClose = () => {},
}) => (
  <div tw="flex flex-col items-center rounded">
    <div tw="w-full flex items-center justify-between py-3 px-6 rounded-t">
      <h3 tw="font-bold text-gray-400">{title}</h3>
      {closable &&
        <Button onToggle={onClose}>
          <span tw="text-sm">X</span>
        </Button>
      }
    </div>
    {children}
  </div>
);

export default Card;
