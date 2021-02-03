import React, { FunctionComponent } from "react";

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
  <div className="flex flex-col items-center rounded">
    <div className="w-full flex items-center justify-between py-3 px-6 rounded-t">
      <h3 className="font-bold text-gray-400">{title}</h3>
      {closable &&
        <Button onToggle={onClose}>
          <span className="text-sm">X</span>
        </Button>
      }
    </div>
    {children}
  </div>
);

export default Card;
