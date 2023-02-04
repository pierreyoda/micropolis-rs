import React, { FunctionComponent } from "react";

export interface TextInputProps {
  value: string;
  onChange: (newValue: string) => void;
  placeholder?: string;
}

const TextInput: FunctionComponent<TextInputProps> = ({ value, onChange, placeholder = "" }) => (
  <input
    className="custom-input"
    tabIndex={0}
    type="text"
    value={value}
    onChange={e => onChange(e.target.value)}
    placeholder={placeholder}
  />
);

export default TextInput;
