import React, { FunctionComponent } from "react";
import "twin.macro";

import Nav from "@/components/Nav";
import Head from "@/components/Head";

const MainLayout: FunctionComponent = ({ children }) => (
  <div tw="w-screen h-screen flex flex-col">
    <Head
      title="micropolis-rs"
      description="Rewrite of Micropolis (open-source classic Sim City) in Rust and Typescript"
      url="TODO:"
      ogImage="TODO:"
    />
    <Nav />
    <main tw="flex-grow">
      {children}
    </main>
  </div>
);

export default MainLayout;
