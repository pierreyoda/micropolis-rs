import React, { useCallback, FunctionComponent } from "react";
import tw from "tailwind.macro";
import styled from "@emotion/styled";

export interface TextInputProps {
  value: string;
  onChange: (newValue: string) => void;
  placeholder?: string;
}

const CustomInput = styled.input`
  ${tw`w-full block appearance-none leading-normal`};
  ${tw`bg-white border border-gray-300 rounded-lg py-2 px-4`};
  &:focus {
    ${tw`outline-none shadow-outline`};
  }
`;

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
