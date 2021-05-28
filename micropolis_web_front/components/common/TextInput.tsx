import React, { useCallback, FunctionComponent } from "react";

export interface TextInputProps {
  value: string;
  onChange: (newValue: string) => void;
  placeholder?: string;
}

const TextInput: FunctionComponent<TextInputProps> = ({
  value,
  onChange,
  placeholder = "",
}) => (
  <input
    className="custom-input"
    tabIndex={0}
    type="text"
    value={value}
    onChange={useCallback((e) => onChange(e.target.value), [onChange])}
    placeholder={placeholder}
  />
);

export default TextInput;
