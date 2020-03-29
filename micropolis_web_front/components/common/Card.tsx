import React, { FunctionComponent, useState, useCallback, useMemo } from "react";
import Color from "color";

import Button from "./Button";

export interface CardProps {
  title: string;
  closable?: boolean;
  onClose?: () => void;
  backgroundColor: string;
  className?: string;
}

const Card: FunctionComponent<CardProps> = ({
  children,
  title,
  closable = false,
  onClose = () => {},
  backgroundColor,
}) => {
  const headerColor = useMemo(() => new Color(backgroundColor).darken(0.3).hex(), [backgroundColor]);

  return (
    <div
      className="flex flex-col items-center rounded"
      style={{ backgroundColor }}
    >
      <div
        className="w-full flex items-center justify-between py-3 px-6 rounded-t"
        style={{ backgroundColor: headerColor }}
      >
        <h3 className="font-bold text-gray-400">{title}</h3>
        {closable &&
          <Button
            width="25px"
            height="25px"
            color="red"
            onToggle={onClose}
          >
            <span className="text-sm">X</span>
          </Button>
        }
      </div>
      {children}
    </div>
  );
};

export default Card;
