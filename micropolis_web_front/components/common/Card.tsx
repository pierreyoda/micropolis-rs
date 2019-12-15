import React, { FunctionComponent } from "react";

const Card: FunctionComponent = ({ children }) => (
  <div className="flex flex-col items-center">
    {children}
  </div>
);

export default Card;
