import React, { FunctionComponent } from "react";
import Loader from "react-loader-spinner";

export const loaderSpinnerTypes = [
  "Audio",
  "BallTriangle",
  "Bars",
  "Circles",
  "Grid",
  "Hearts",
  "MutatingDots",
  "Oval",
  "Plane",
  "Puff",
  "RevolvingDot",
  "Rings",
  "TailSpin",
  "ThreeDots",
  "Triangle",
  "Watch",
];

export type LoaderSpinnerType = typeof loaderSpinnerTypes[number];

export interface LoaderSpinnerProps {
  type: LoaderSpinnerType;
  visible?: boolean;
  width?: number;
  height?: number;
  color?: string;
  timeout?: number;
}

/**
 * @see https://github.com/mhnpd/react-loader-spinner
 */
const LoaderSpinner: FunctionComponent<LoaderSpinnerProps> = props => (
  <Loader {...props} />
);

export default LoaderSpinner;
