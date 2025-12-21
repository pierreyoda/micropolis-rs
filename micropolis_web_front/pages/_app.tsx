import NextApp from "next/app";

import "@/styles/globals.scss";
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
