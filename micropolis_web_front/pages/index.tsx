import React from "react";
import dynamic from "next/dynamic";

import LoaderSpinner from "@/components/common/LoaderSpinner";

const PixiContainerLoader = () => (
  <div className="w-full h-full flex flex-col items-center justify-center p-12">
    <LoaderSpinner type="Watch" />
  </div>
);

const PixiContainer = dynamic(
  () => import("@/components/game/PixiContainer"),
  { ssr: false, loading: PixiContainerLoader },
);

const Home = () => (
  <div className="w-full h-full flex flex-col items-center justify-center">
    <PixiContainer debug />
  </div>
);

export default Home;
