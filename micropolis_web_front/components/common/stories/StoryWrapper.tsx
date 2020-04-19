import React, { FunctionComponent } from "react";

interface StoryWrapperProps {
  full?: boolean;
  noPadding?: boolean;
}

const StoryWrapper: FunctionComponent<StoryWrapperProps> = ({
  children,
  full = true,
  noPadding = false,
}) => (
  <div
    className={`classflex flex-col items-center justify-center ${noPadding ? "" : "p-12"} bg-gray-200`}
    style={{ width: full ? "100%" : "1280px", height: full ? "100vh" : "800px" }}
  >
    {children}
  </div>
);

export default StoryWrapper;
