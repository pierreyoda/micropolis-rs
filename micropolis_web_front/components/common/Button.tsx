import React, {
  FunctionComponent,
  KeyboardEvent,
  MouseEvent,
  useCallback,
  useMemo,
} from "react";
import tw from "tailwind.macro";
import styled from "@emotion/styled";

import { ENTER } from "@/utils/keys";

const buttonColors = [
  "gray",
  "red",
  "orange",
  "yellow",
  "green",
  "teal",
  "blue",
  "indigo",
  "purple",
  "pink",
] as const;
export type ButtonColor = typeof buttonColors[number];

const ButtonContainer = styled.div`
  ${tw`py-2 px-4 rounded outline-none`};
  ${tw`font-bold text-center text-white`};
  ${tw`flex items-center justify-center`};
`;

export interface ButtonProps {
  disabled?: boolean;
  width?: string;
  height?: string;
  color?: ButtonColor;
  active: boolean;
  onToggle: () => void;
}

const isKeyboardEvent = (e: KeyboardEvent | MouseEvent): e is KeyboardEvent =>
  !!(e as KeyboardEvent).key;

const Button: FunctionComponent<ButtonProps> = ({
  disabled = false,
  active = false,
  width = "100px",
  height = "50px",
  color = "green",
  onToggle,
  children,
}) => {
  const trigger = useCallback(
    (e: KeyboardEvent | MouseEvent, type?: "mousedown" | "mouseup") => {
      if (disabled) { return; }
      if (isKeyboardEvent(e)) {
        if (e.key !== ENTER) { return; }
        onToggle();
      }
      if (type === "mousedown") { return; }
      onToggle();
    },
    [disabled],
  );

  const className = useMemo(
    () => disabled
      ? "bg-gray-500 font-italic"
      : active
        ? `bg-${color}-700`
        : `bg-${color}-500 hover:bg-${color}-600`,
    [disabled, active, color],
  );

  return (
    <ButtonContainer
      role="button"
      tabIndex={0}
      aria-disabled={disabled}
      aria-pressed={active}
      onMouseDown={e => trigger(e, "mousedown")}
      onMouseUp={e => trigger(e, "mouseup")}
      onKeyDown={trigger}
      className={className}
      style={{ width, height }}
    >
      {children}
    </ButtonContainer>
  );
};

export default Button;
