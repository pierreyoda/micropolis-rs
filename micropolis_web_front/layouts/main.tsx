import React, { FunctionComponent } from "react";

import Nav from "@/components/nav";
import Head from "@/components/head";

const MainLayout: FunctionComponent = ({ children }) => (
  <div>
    <Head
      title="micropolis-rs"
      description="Rewrite of Micropolis (open-source classic Sim City) in Rust and Typescript"
      url="TODO:"
      ogImage="TODO:"
    />
    <Nav />
    <main>
      {children}
    </main>
  </div>
);

export default MainLayout;
