import React, { FunctionComponent, useCallback } from "react";

export interface CheckboxInputProps {
  label: string;
  checked: boolean;
  onToggle: (checked: boolean) => void;
  className?: string;
  disabled?: boolean;
  size?: string;
}

const CheckboxInput: FunctionComponent<CheckboxInputProps> = ({
  label,
  checked,
  onToggle,
  disabled = false,
  size = "16px",
  className
}) => (
  <label className="flex flex-row items-center">
    <div className="flex items-center">
      <input
        disabled={disabled}
        type="checkbox"
        style={{
          border: 0,
          clip: "rect(0 0 0 0)",
          clipPath: "inset(50%)",
          height: "1px",
          margin: -"1px",
          overflow: "hidden",
          padding: 0,
          position: "absolute",
          whiteSpace: "nowrap",
          width: "1px",
        }}
        checked={checked}
        onChange={useCallback(e => onToggle(e.target.checked), [onToggle])}
      />
    </div>
    <span className="ml-2">{label}</span>
  </label>
);

export default CheckboxInput;
