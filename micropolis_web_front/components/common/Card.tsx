import React, { FunctionComponent } from "react";

export interface CardProps {
  title: string;
  closable?: boolean;
  backgroundColor: string;
}

const Card: FunctionComponent<CardProps> = ({ 
  children, 
  title,
  closable = false,
  backgroundColor,
}) => (
  <div 
    className="flex flex-col items-center"
    css={{ backgroundColor }}
  >
    {children}
  </div>
);

export default Card;
