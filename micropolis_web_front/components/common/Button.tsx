import React, {
  useCallback,
  FunctionComponent,
} from "react";

import { ENTER } from "@/utils/keys";

export interface ButtonProps {
  disabled?: boolean;
  onToggle: () => void;
}

const Button: FunctionComponent<ButtonProps> = ({
  disabled = false,
  onToggle,
  children,
}) => (
  <div
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
  </div>
);

export default Button;
