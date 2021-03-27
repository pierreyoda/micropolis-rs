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
    <div className="flex items-center justify-between w-full px-6 py-3 rounded-t">
      <h3 className="font-bold text-gray-400">{title}</h3>
      {closable && (
        <Button onToggle={onClose}>
          <span className="text-sm">X</span>
        </Button>
      )}
    </div>
    {children}
  </div>
);

export default Card;
