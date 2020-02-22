import React, { FunctionComponent } from "react";

const StoryWrapper: FunctionComponent = ({ children }) => (
  <div
    className="flex flex-col items-center justify-center p-12 bg-gray-500"
    css={{ width: "1280px", height: "800px" }}
  >
    {children}
  </div>
);

export default StoryWrapper;
