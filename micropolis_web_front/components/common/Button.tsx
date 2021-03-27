import React, { FunctionComponent, useMemo } from "react";

interface ButtonProps {
  onToggle: () => void;
  disabled?: boolean;
}

const Button: FunctionComponent<ButtonProps> = ({
  onToggle,
  children,
  disabled,
  ...props
}) => {
  const onClick = useMemo(() => (disabled ? () => {} : onToggle), [
    disabled,
    onToggle,
  ]);

  return (
    <button
      className="custom-button"
      onClick={onClick}
      disabled={disabled}
      {...props}
    >
      {children}
    </button>
  );
};

export default Button;
