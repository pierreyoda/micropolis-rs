import React, { useCallback, FunctionComponent } from "react";

export interface TextInputProps {
  value: string;
  onChange: (newValue: string) => void;
  placeholder?: string;
}

const inputClasses =
  "w-full block appearance-none leading-normal" +
  "bg-white border border-gray-300 rounded-lg py-2 px-4";

const TextInput: FunctionComponent<TextInputProps> = ({
  value,
  onChange,
  placeholder = "",
}) => (
  <input
    tabIndex={0}
    type="text"
    value={value}
    onChange={useCallback(e => onChange(e.target.value), [onChange])}
    placeholder={placeholder}
    className={`${inputClasses} focus:outline-none focus:shadow-outline`}
  />
);

export default TextInput;
