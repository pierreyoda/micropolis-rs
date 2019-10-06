import React from "react";
import Link from "next/link";
import tw from "tailwind.macro";
import styled from "@emotion/styled";

const NavContainer = styled.nav`
  ${tw`w-full flex items-center justify-between`};
  ${tw`py-4 px-12 bg-green-900`};
  ${tw`font-bold text-white`};
`;

const NavLinkListItem = styled.li`
  ${tw`mr-8`};
  &:last-of-type {
    ${tw`mr-0`};
  }
`;

const Nav = () => (
  <NavContainer>
    <h1 className="text-2xl">
      <Link href="/">
        <a>micropolis-rs</a>
      </Link>
    </h1>
    <ul className="flex items-center">
      <NavLinkListItem>
        <Link href="/about">
          <a>About</a>
        </Link>
      </NavLinkListItem>
      <NavLinkListItem>
        <Link href="https://github.com/pierreyoda/micropolis-rs">
          <a>GitHub</a>
        </Link>
      </NavLinkListItem>
    </ul>
  </NavContainer>
);

export default Nav;
