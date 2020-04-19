import React, { FunctionComponent, useCallback } from "react";
import { css } from "@emotion/core";
import styled from "@emotion/styled";
import tw from "tailwind.macro";

const CheckboxContainer = styled.div`
  ${tw`inline-block flex items-center`};
`;

const Icon = styled.svg`
  fill: none;
  stroke: white;
  stroke-width: 2px;
`;

const checkboxCss = css`
  border: 0;
  clip: rect(0 0 0 0);
  clippath: inset(50%);
  height: 1px;
  margin: -1px;
  overflow: hidden;
  padding: 0;
  position: absolute;
  white-space: nowrap;
  width: 1px;
`

const StyledCheckbox = styled.div`
  ${tw`inline-block border-sm`};
  transition: all 150ms;
`;

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
    <CheckboxContainer className={className}>
      <input
        disabled={disabled}
        type="checkbox"
        css={checkboxCss}
        checked={checked}
        onChange={useCallback(e => onToggle(e.target.checked), [onToggle])}
      />
    </CheckboxContainer>
    <span className="ml-2">{label}</span>
  </label>
);

export default CheckboxInput;
