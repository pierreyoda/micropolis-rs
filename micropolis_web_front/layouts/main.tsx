import { FunctionComponent, ReactNode } from "react";

import Nav from "@/components/Nav";
import Head from "@/components/Head";

interface MainLayoutProps {
  children?: ReactNode;
}

const MainLayout: FunctionComponent<MainLayoutProps> = ({ children }) => (
  <div className="w-screen h-screen flex flex-col">
    <Head
      title="micropolis-rs"
      description="Rewrite of Micropolis (open-source classic Sim City) in Rust and Typescript"
      url="TODO:"
      ogImage="TODO:"
    />
    <Nav />
    <main className="flex-grow">{children}</main>
  </div>
);

export default MainLayout;
