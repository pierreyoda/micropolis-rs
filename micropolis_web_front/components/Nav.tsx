import React from "react";
import Link from "next/link";

const Nav = () => (
  <nav className="flex items-center justify-between w-full px-2 py-4 font-bold text-white bg-green-900">
    <h1 className="text-2xl">
      <Link href="/">micropolis-rs</Link>
    </h1>
    <ul className="flex items-center">
      <li className="mr-8">
        <Link href="/about">About</Link>
      </li>
      <li className="mr-0">
        <Link href="https://github.com/pierreyoda/micropolis-rs">GitHub</Link>
      </li>
    </ul>
  </nav>
);

export default Nav;
