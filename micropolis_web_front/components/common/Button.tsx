import React, { FunctionComponent, useMemo } from "react";
import tw, { styled } from "twin.macro";

interface ButtonProps {
  onToggle: () => void;
  disabled?: boolean;
}

const StyledButton = styled.button(({
  disabled,
 }: ButtonProps) => [
  tw`px-8 py-2 rounded focus:outline-none text-lg text-white`,
  tw`bg-blue-500 ring hocus:ring-4`,
  tw`transform transition-transform transition-colors duration-200`,
  tw`hocus:(scale-105 text-gray-200)`,

  disabled && tw`ring-0 bg-gray-200`,
]);

const Button: FunctionComponent<ButtonProps> = ({
  onToggle,
  children,
  disabled,
  ...props
}) => {
  const onClick = useMemo(
    () => disabled ? () => {} : onToggle,
    [disabled, onToggle],
  );

  return (
    <StyledButton onClick={onClick} disabled={disabled} {...props}>
      {children}
    </StyledButton>
  );
};

export default Button;
