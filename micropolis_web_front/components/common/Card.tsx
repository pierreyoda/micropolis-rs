import clsx from "clsx";
import { FunctionComponent, ReactNode } from "react";

import Button from "./Button";

export interface CardProps {
  title: string;
  closable?: boolean;
  className?: string;
  onClose?: () => void;
  children?: ReactNode;
}

const Card: FunctionComponent<CardProps> = ({ children, title, className, closable = false, onClose = () => {} }) => (
  <div className={clsx("flex flex-col items-center rounded", className)}>
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
