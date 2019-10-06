import React from "react";
import NextApp from "next/app";

import "@/assets/styles/tailwind.css";
import MainLayout from "@/layouts/main";

export default class App extends NextApp {
  render() {
    const { Component, pageProps } = this.props;
    return (
      <MainLayout>
        <Component {...pageProps} />
      </MainLayout>
    );
  }
}
