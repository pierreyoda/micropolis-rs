import Head from "next/head";
import { FunctionComponent, ReactNode } from "react";

import "./globals.css";
import Nav from "@/components/Nav";

const RootLayout: FunctionComponent<{ children: ReactNode }> = ({ children }) => (
  <html lang="en" className="w-screen h-screen">
    <Head>
      {/* TODO: meta images etc. */}
      <title>micropolis-rs</title>
      <meta name="viewport" content="width=device-width, initial-scale=1" />
    </Head>
    <body className="flex flex-col">
      <Nav />
      <main className="flex-grow">{children}</main>
    </body>
  </html>
);

export default RootLayout;
