import clsx from "clsx";
import React, { FunctionComponent, useMemo } from "react";

interface ButtonProps {
  onToggle: () => void;
  disabled?: boolean;
  className?: string;
}

const Button: FunctionComponent<ButtonProps> = ({ onToggle, disabled, className, children }) => {
  const onClick = useMemo(() => (disabled ? () => {} : onToggle), [disabled, onToggle]);

  return (
    <button className={clsx("custom-button", className)} onClick={onClick} disabled={disabled}>
      {children}
    </button>
  );
};

export default Button;
