import React, {
  useCallback,
  FunctionComponent,
  KeyboardEvent,
  MouseEvent,
  useMemo,
} from "react";
import tw from "tailwind.macro";
import styled from "@emotion/styled";

import { ENTER } from "@/utils/keys";

const ButtonContainer = styled.div`
  ${tw`py-2 px-4 rounded outline-none`};
  ${tw`bg-green-500 hover:bg-green-500`};
  ${tw`font-bold text-center text-white`};
`;

export interface ButtonProps {
  disabled?: boolean;
  width?: string;
  active: boolean;
  onToggle: () => void;
}

const isKeyboardEvent = (e: KeyboardEvent | MouseEvent): e is KeyboardEvent =>
  !!(e as KeyboardEvent).key;

const Button: FunctionComponent<ButtonProps> = ({
  disabled = false,
  active = false,
  width = "100px",
  onToggle,
  children,
}) => {
  const trigger = useCallback(
    (e: KeyboardEvent | MouseEvent) => {
      if (disabled) { return; }
      if (isKeyboardEvent(e) && e.key !== ENTER) { return; }
      onToggle();
    },
    [disabled],
  );

  const className = useMemo(
    () => disabled 
      ? "bg-gray-500"
      : active
        ? "bg-green-700"
        : "bg-green-500 hover:bg-green-600",
    [disabled, active],
  );

  return (
    <ButtonContainer
      role="button"
      tabIndex={0}
      aria-disabled={disabled}
      aria-pressed={active}
      onClick={trigger}
      onKeyDown={trigger}
      className={className}
      css={ width }
    >
      {children}
    </ButtonContainer>
  );
};

  const className = useMemo(
    () => disabled 
      ? "bg-gray-500"
      : active
        ? "bg-green-700"
        : "bg-green-500 hover:bg-green-600",
    [disabled, active],
  );

  return (
    <ButtonContainer
      role="button"
      tabIndex={0}
      aria-disabled={disabled}
      aria-pressed={active}
      onClick={trigger}
      onKeyDown={trigger}
      className={className}
      css={width}
    >
      {children}
    </ButtonContainer>
  );
};

export default Button;
