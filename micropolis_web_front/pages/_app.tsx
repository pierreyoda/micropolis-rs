import React from "react";
import NextApp from "next/app";
import { GlobalStyles } from "twin.macro";

import MainLayout from "@/layouts/main";

export default class App extends NextApp {
  render() {
    const { Component, pageProps } = this.props;
    return (
      <MainLayout>
        <GlobalStyles />
        <Component {...pageProps} />
      </MainLayout>
    );
  }
}
