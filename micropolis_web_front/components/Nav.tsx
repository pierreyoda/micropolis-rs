import React from "react";
import Link from "next/link";

const Nav = () => (
  <nav className="w-full flex items-center justify-between py-4 px-2 bg-green-900 font-bold text-white">
    <h1 className="text-2xl">
      <Link href="/">
        <a>micropolis-rs</a>
      </Link>
    </h1>
    <ul className="flex items-center">
      <li className="mr-8">
        <Link href="/about">
          <a>About</a>
        </Link>
      </li>
      <li className="mr-0">
        <Link href="https://github.com/pierreyoda/micropolis-rs">
          <a>GitHub</a>
        </Link>
      </li>
    </ul>
  </nav>
);

export default Nav;
