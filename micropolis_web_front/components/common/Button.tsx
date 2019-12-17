import React, {
  useCallback,
  FunctionComponent,
} from "react";
import tw from "tailwind.macro";
import styled from "@emotion/styled";

import { ENTER } from "@/utils/keys";

const ButtonContainer = styled.div`
  ${tw`py-2 px-4 rounded`};
  ${tw`bg-green-500 hover:bg-green-500`};
  ${tw`font-bold text-center text-white`};
`;

export interface ButtonProps {
  disabled?: boolean;
  onToggle: () => void;
}

const Button: FunctionComponent<ButtonProps> = ({
  disabled = false,
  onToggle,
  children,
}) => (
  <ButtonContainer
    role="button"
    tabIndex={0}
    aria-disabled={disabled}
    onClick={onToggle}
    onKeyDown={useCallback(e => {
      if (e.key === ENTER) {
        onToggle();
      }
    }, [onToggle])}
  >
    {children}
  </ButtonContainer>
);

export default Button;
