import React, { useCallback, FunctionComponent } from "react";
import tw, { styled } from "twin.macro";

const CustomInput = styled.input(() => [
  tw`block w-full leading-normal appearance-none`,
  tw`px-4 py-2 bg-white border border-gray-300 rounded-lg`,
  tw`hocus:(outline-none)`,
]);

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
  <CustomInput
    tabIndex={0}
    type="text"
    value={value}
    onChange={useCallback(e => onChange(e.target.value), [onChange])}
    placeholder={placeholder}
  />
);

export default TextInput;
