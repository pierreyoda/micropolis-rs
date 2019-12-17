import React, { FunctionComponent } from "react";

const StoryWrapper: FunctionComponent = ({ children }) => (
  <div className="w-32 h-full flex flex-col items-center justify-center p-12 bg-gray-500">
    {children}
  </div>
);

export default StoryWrapper;
