import React from "react";
import Link from "next/link";

const Nav = () => (
  <nav>
    <ul>
      <li>
        <Link href="/">
          <a>micropolis-rs</a>
        </Link>
      </li>
      <li>
        <Link href="/about">
          <a>About</a>
        </Link>
        <Link href="https://github.com/pierreyoda/micropolis-rs">
          <a>GitHub</a>
        </Link>
      </li>
    </ul>
  </nav>
);

export default Nav;
