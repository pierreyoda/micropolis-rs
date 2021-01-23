import React, {
  FunctionComponent,
} from "react";

export const buttonColors = [
  "gray",
  "red",
  "orange",
  "yellow",
  "green",
  "teal",
  "blue",
  "indigo",
  "purple",
  "pink",
] as const;
export type ButtonColor = typeof buttonColors[number];

const buttonClasses =
  `py-2 px-4 rounded-lg outline-none appearance-none` +
  `font-bold text-center text-white` +
  `flex items-center justify-center`;

export interface ButtonProps {
  onToggle: () => void;
  disabled?: boolean;
  width?: string;
  height?: string;
  color?: ButtonColor;
  className?: string;
}

const Button: FunctionComponent<ButtonProps> = ({
  disabled = false,
  width = "100px",
  height = "50px",
  color = "green",
  className,
  onToggle,
  children,
}) => (
  <button
    onClick={onToggle}
    disabled={disabled}
    style={{ width, height }}
    className={`${className} ${buttonClasses} bg-${color}-500 hover:bg-${color}-600 focus:bg-${color}-700`}
  >
    {children}
  </button>
);

export default Button;
